<script lang="ts">
  // Import global CSS
  import "../app.css";
  
  // Dark mode state
  let darkMode = false;
  
  function toggleDarkMode() {
    darkMode = !darkMode;
    document.documentElement.classList.toggle('dark', darkMode);
  }
</script>

<div class="min-h-screen bg-ileana-light dark:bg-ileana-dark transition-colors duration-300">
  <!-- Header -->
  <header class="bg-white dark:bg-slate-800 shadow-sm">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center py-4">
        <h1 class="text-2xl font-bold text-ileana-primary dark:text-ileana-accent">Ileana</h1>
        <button
          on:click={toggleDarkMode}
          class="p-2 rounded-full bg-ileana-primary text-white hover:bg-ileana-secondary transition-colors"
          aria-label="Toggle dark mode"
        >
          {darkMode ? '☀️' : '🌙'}
        </button>
      </div>
    </div>
  </header>
  
  <!-- Main content -->
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <slot />
  </main>
  
  <!-- Footer -->
  <footer class="bg-white dark:bg-slate-800 mt-auto py-4">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        © {new Date().getFullYear()} Ileana - Tauri + SvelteKit + Tailwind
      </p>
    </div>
  </footer>
</div>