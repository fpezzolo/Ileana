<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  

  
  let screenshotData = '';
  let isLoading = false;
  let errorMessage = '';
  
  onMount(async () => {
    // Ascolta l'evento per catturare screenshot
    await listen('capture-screenshot', async () => {
      await captureScreenshot();
    });
  });
  
  async function captureScreenshot() {
    try {
      isLoading = true;
      errorMessage = '';
      
      // Carica html2canvas se non è già caricato
      if (!(window as any).html2canvas) {
        const script = document.createElement('script');
        script.src = 'https://html2canvas.hertzen.com/dist/html2canvas.min.js';
        document.head.appendChild(script);
        
        await new Promise((resolve) => {
          script.onload = resolve;
          script.onerror = () => {
            errorMessage = 'Failed to load html2canvas library';
            isLoading = false;
          };
        });
      }
      
      // Cattura lo screenshot usando html2canvas
      const canvas = await (window as any).html2canvas(document.body, {
        logging: true,
        useCORS: true,
        scale: 1,
        allowTaint: true
      });
      
      // Converti in base64
      screenshotData = canvas.toDataURL('image/png');
      
      // Salva automaticamente il file
      const link = document.createElement('a');
      link.download = `screenshot-${new Date().toISOString().slice(0, 19)}.png`;
      link.href = screenshotData;
      link.click();
      
      console.log('Screenshot captured and downloaded!');
      
    } catch (error: unknown) {
      const err = error as Error;
      errorMessage = `Error capturing screenshot: ${err.message}`;
      console.error('Screenshot error:', error);
    } finally {
      isLoading = false;
    }
  }
  
  async function triggerScreenshot() {
    try {
      // Chiamata al backend per triggerare l'evento
      const result = await invoke('capture_screenshot');
      console.log('Backend response:', result);
    } catch (error: unknown) {
      const err = error as Error;
      errorMessage = `Backend error: ${err.message}`;
      console.error('Backend error:', error);
    }
  }
</script>

<div class="screenshot-container">
  <button 
    on:click={triggerScreenshot}
    disabled={isLoading}
    class="screenshot-button"
  >
    {isLoading ? 'Capturing...' : '📸 Capture Screenshot'}
  </button>
  
  {#if errorMessage}
    <div class="error-message">
      ❌ {errorMessage}
    </div>
  {/if}
  
  {#if screenshotData && !isLoading}
    <div class="screenshot-preview">
      <h3>Screenshot Preview:</h3>
      <img src={screenshotData} alt="Screenshot preview" class="preview-image" />
    </div>
  {/if}
</div>

<style>
  .screenshot-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .screenshot-button {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }
  
  .screenshot-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }
  
  .screenshot-button:disabled {
    background: #cbd5e0;
    cursor: not-allowed;
  }
  
  .error-message {
    color: #e53e3e;
    background: #fed7d7;
    padding: 0.75rem;
    border-radius: 4px;
    font-size: 0.875rem;
  }
  
  .screenshot-preview {
    margin-top: 1rem;
  }
  
  .preview-image {
    max-width: 100%;
    border: 1px solid #e2e8f0;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }
</style>