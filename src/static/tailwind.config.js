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
      animation: {
        "blink-color": "blink-color 2s infinite",
      },
      keyframes: {
        "blink-color": {
          "0%, 100%": { color: "#FFD700" }, // Gold
          "33%": { color: "#FF4500" }, // OrangeRed
          "66%": { color: "#00FF00" }, // Lime
        },
      },
    },
  },
  plugins: [],
};
