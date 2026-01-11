<script lang="ts">
    import Quadrato from '../components/Quadrato.svelte';  
    import Rettangolo from '../components/Rettangolo.svelte';
    import Cerchio from '../components/Cerchio.svelte';
    import Triangolo from '../components/Triangolo.svelte';
    import Card from '../components/Card.svelte';
</script>

<!-- Hero Section -->
<section class="text-center py-12 mb-12">
  <h1 class="text-4xl md:text-5xl font-bold text-ileana-primary dark:text-ileana-accent mb-4">
    Benvenuto in Ileana
  </h1>
  <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto mb-8">
    Applicazione desktop moderna costruita con Tauri, SvelteKit e Tailwind CSS
  </p>
  
  <!-- Tecnologie -->
  <div class="flex justify-center items-center space-x-8 mb-12">
    <div class="text-center">
      <img src="/tauri.svg" class="h-16 w-16 mx-auto mb-2" alt="Tauri" />
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Tauri</span>
    </div>
    <div class="text-center">
      <img src="/svelte.svg" class="h-16 w-16 mx-auto mb-2" alt="SvelteKit" />
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300">SvelteKit</span>
    </div>
    <div class="text-center">
      <img src="/vite.svg" class="h-16 w-16 mx-auto mb-2" alt="Vite" />
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Vite</span>
    </div>
  </div>
</section>

<!-- Features Section -->
<section class="mb-12">
  <h2 class="text-2xl font-bold text-ileana-primary dark:text-ileana-accent mb-8 text-center">
    Componenti Geometrici
  </h2>
  
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <!-- Quadrato predefinito -->
    <Card
      title="Quadrato Standard"
      description="Versione predefinita con colori standard"
      color="primary"
    >
      <div class="flex justify-center">
        <Quadrato />
      </div>
    </Card>
    
  
    
    <!-- Rettangolo predefinito -->
    <Card
      title="Rettangolo Standard"
      description="Versione predefinita con colori standard"
      color="primary"
    >
      <div class="flex justify-center">
        <Rettangolo />
      </div>
    </Card>
    
     
    <!-- Cerchio predefinito -->
    <Card
      title="Cerchio Standard"
      description="Versione predefinita con colori standard"
      color="primary"
    >
      <div class="flex justify-center">
        <Cerchio />
      </div>
    </Card>
    
    <!-- Triangolo predefinito -->
    <Card
      title="Triangolo Standard"
      description="Versione predefinita con colori standard"
      color="primary"
    >
      <div class="flex justify-center">
        <Triangolo />
      </div>
    </Card>
     
  </div>
</section>

<!-- Call to Action -->
<section class="text-center py-8">
  <button class="bg-ileana-primary hover:bg-ileana-secondary text-white font-medium py-3 px-6 rounded-lg transition-colors">
    Esplora di più
  </button>
</section>