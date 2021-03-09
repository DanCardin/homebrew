module.exports = {
  purge: {
    content: ["./public/**/*.html", "./src/**/*.vue"],
    options: {
      extractors: ["purgecss-from-pug"],
    },
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [],
};
