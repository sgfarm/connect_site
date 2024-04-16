/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      fontFamily: {
        'anton': [ 'anton', 'ui-sans-serif', 'system-ui', 'sans-serif', ],
  			'sans': [
          'inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji",
          "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
        ],
      },
    },
  },
  plugins: [],
}
