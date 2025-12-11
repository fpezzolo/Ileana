<script lang="ts">
  export let title: string;
  export let description: string;
  export let color: 'primary' | 'secondary' | 'accent' | 'success' | 'warning' | 'danger' = 'primary';
  export let fullWidth: boolean = false;
</script>

<div class={`bg-white dark:bg-slate-700 p-6 rounded-lg shadow-sm border border-gray-200 dark:border-slate-600 ${fullWidth ? 'col-span-full' : ''}`}>
  <div class="flex items-start justify-between mb-4">
    <h3 class="text-lg font-semibold text-ileana-primary dark:text-ileana-accent">
      {title}
    </h3>
    <span class={`px-2 py-1 rounded-full text-xs font-medium ${
      color === 'primary' ? 'bg-ileana-primary/10 text-ileana-primary' :
      color === 'secondary' ? 'bg-ileana-secondary/10 text-ileana-secondary' :
      color === 'accent' ? 'bg-ileana-accent/10 text-ileana-accent' :
      color === 'success' ? 'bg-ileana-success/10 text-ileana-success' :
      color === 'warning' ? 'bg-ileana-warning/10 text-ileana-warning' :
      'bg-ileana-danger/10 text-ileana-danger'
    }`}>
      {color.toUpperCase()}
    </span>
  </div>
  <p class="text-sm text-gray-600 dark:text-gray-300">
    {description}
  </p>
  <div class="mt-4">
    <slot />
  </div>
</div>