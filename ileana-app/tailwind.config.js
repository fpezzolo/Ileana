/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
  ],
  darkMode: 'class', // Abilita dark mode via classe
  theme: {
    extend: {
      colors: {
        // Tema personalizzato Ileana
        'ileana': {
          'primary': '#6366f1', // Indigo 500
          'secondary': '#8b5cf6', // Violet 500
          'accent': '#ec4899', // Pink 500
          'success': '#10b981', // Emerald 500
          'warning': '#f59e0b', // Amber 500
          'danger': '#ef4444', // Red 500
          'light': '#f8fafc', // Slate 50
          'dark': '#0f172a', // Slate 900
        },
      },
      fontFamily: {
        'sans': ['Inter', 'ui-sans-serif', 'system-ui'],
        'serif': ['ui-serif', 'Georgia'],
        'mono': ['ui-monospace', 'SFMono-Regular'],
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}