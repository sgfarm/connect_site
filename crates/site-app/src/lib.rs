use leptos::*;
use leptos_meta::*;
use leptos_router::{ActionForm, Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Style>{include_str!("../style/fonts.css")}</Style>
    <Stylesheet id="leptos" href="/pkg/site.css"/>
    <leptos_meta::Link rel="preload" href="/fonts/inter.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />
    <leptos_meta::Link rel="preload" href="/fonts/anton.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />

    <Title text="Solid Ground Connect"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <Router>
      <Routes>
        <Route path="/" view=ConnectForm />
        <Route path="/thankyou" view=ThankYouPage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn PageWrapper(children: Children) -> impl IntoView {
  view! {
    <section class="bg-white">
      <div class="lg:grid lg:min-h-screen lg:grid-cols-12">
        <aside class="relative block h-16 lg:order-last lg:col-span-5 lg:h-full xl:col-span-6">
          <img
            alt=""
            src="https://images.unsplash.com/photo-1605106702734-205df224ecce?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=870&q=80"
            class="absolute inset-0 h-full w-full object-cover"
          />
        </aside>

        <main
          class="flex items-center justify-center px-8 py-8 sm:px-12 lg:col-span-7 lg:px-16 lg:py-12 xl:col-span-6"
        >
          {children()}
        </main>
      </div>
    </section>
  }
}

#[component]
pub fn ThankYouPage() -> impl IntoView {
  view! {
    <PageWrapper>
      <p class="text-xl max-w-prose">
        "Thank you so much for sending us your information. We'll be in contact soon. Have a blessed day!"
      </p>
    </PageWrapper>
  }
}

#[component]
pub fn ConnectForm() -> impl IntoView {
  let upload_contact = create_server_action::<UploadContact>();

  view! {
    <PageWrapper>
      <div class="max-w-xl lg:max-w-3xl">
        <a class="block" href="https://solidgroundfarm.org" target="_blank" referer="none">
          <p class="font-anton text-4xl">SGF</p>
        </a>

        <h1 class="mt-6 text-2xl font-bold text-gray-900 sm:text-3xl md:text-4xl">
          "Connect with Solid Ground!"
        </h1>

        <p class="mt-4 text-gray-500">
          "Provide your information below and we'll be in touch. Thank you so much for connecting with Solid Ground!"
        </p>

        <ActionForm action=upload_contact class="mt-8 grid grid-cols-6 gap-6">
          <div class="col-span-6 sm:col-span-3">
            <label for="FirstName" class="block text-sm font-medium text-gray-700">
              "First Name"
            </label>

            <input
              type="text"
              id="FirstName"
              name="first_name"
              class="mt-1 w-full rounded-md border-gray-400 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="col-span-6 sm:col-span-3">
            <label for="LastName" class="block text-sm font-medium text-gray-700">
              "Last Name"
            </label>

            <input
              type="text"
              id="LastName"
              name="last_name"
              class="mt-1 w-full rounded-md border-gray-400 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="col-span-6">
            <label for="Email" class="block text-sm font-medium text-gray-700">"Email"</label>

            <input
              type="email"
              id="Email"
              name="email"
              class="mt-1 w-full rounded-md border-gray-400 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="col-span-6">
            <label for="PhoneNumber" class="block text-sm font-medium text-gray-700">"Phone Number"</label>

            <input
              type="tel"
              id="PhoneNumber"
              name="phone"
              class="mt-1 w-full rounded-md border-gray-400 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="col-span-6 row-span-2">
            <label for="Note" class="block text-sm font-medium text-gray-700">"Optional Note"</label>

            <textarea
              id="Note"
              name="note"
              class="mt-1 w-full rounded-md border-gray-400 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="col-span-6">
            <button
              class="w-full shrink-0 rounded-md border border-blue-600 bg-blue-600 px-12 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500"
            >
              "Submit your Information"
            </button>
          </div>
        </ActionForm>
      </div>
    </PageWrapper>
  }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Contact {
  first_name: Option<String>,
  last_name:  Option<String>,
  email:      Option<String>,
  phone:      Option<String>,
  note:       Option<String>,
}

fn handle_error(e: impl std::fmt::Debug) -> ServerFnError {
  let error = format!("failed to upload contact: {e:?}");
  logging::error!("{}", error);
  ServerFnError::new(error)
}

#[server]
pub async fn upload_contact(
  first_name: Option<String>,
  last_name: Option<String>,
  email: Option<String>,
  phone: Option<String>,
  note: Option<String>,
) -> Result<(), ServerFnError> {
  let contact = Contact {
    first_name,
    last_name,
    email,
    phone,
    note,
  };

  let contents = serde_json::to_string(&contact).map_err(handle_error)?;

  use object_store::{
    aws::{AmazonS3, AmazonS3Builder},
    path::Path,
    ObjectStore,
  };

  let store: AmazonS3 = AmazonS3Builder::from_env()
    .with_bucket_name("sgfarm-connect")
    .build()
    .map_err(handle_error)?;
  store
    .put(
      &Path::from(format!("/{}.json", ulid::Ulid::new())),
      contents.into(),
    )
    .await
    .map_err(handle_error)?;

  leptos_axum::redirect("/thankyou");

  Ok(())
}
