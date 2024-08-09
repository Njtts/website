/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../*.rs"],

  theme: {
    extend: {
      fontFamily: {
        roboto: ["Roboto", "sans-serif"],
        taviraj: ['"Taviraj"', "serif"],
      },
      colors: {},
      backgroundImage: {
        "vertical-to-pink":
          "linear-gradient(to bottom, white, rgb(248,201,212))",
        "pink-to-white": "linear-gradient(to bottom, rgb(248,201,212), white)",
      },
    },
  },
  plugins: [],
};
