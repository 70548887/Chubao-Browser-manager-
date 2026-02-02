/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: "#2563eb",
        secondary: "#3b82f6",
        "background-light": "#f8fafc",
        "background-dark": "#0f172a",
        "surface-light": "#ffffff",
        "surface-dark": "#1e293b",
        "border-light": "#e2e8f0",
        "border-dark": "#334155",
      },
      fontFamily: {
        display: ["Inter", "PingFang SC", "Microsoft YaHei", "sans-serif"],
        sans: ["Inter", "PingFang SC", "Microsoft YaHei", "sans-serif"],
      },
      borderRadius: {
        DEFAULT: "0.5rem",
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
