/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../*.rs"],

  theme: {
    extend: {
      fontFamily: {
        roboto: ["Roboto", "sans-serif"],
      },
      colors: {},
      backgroundImage: {
        "vertical-to-pink":
          "linear-gradient(to bottom, white, rgb(248,201,212))",
      },
    },
  },
  plugins: [],
};
