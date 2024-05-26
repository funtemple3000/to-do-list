<script>
  import Home from './lib/Home.svelte';
  import Edit from "./lib/Edit.svelte";
  

  let file = null;
  let filePath = null;

  function handleCreateOrOpenSubmit(event){
    file = event.detail.value.jsonData;
    filePath = event.detail.value.filePath;
  }
</script>



<main class="container">
  <h1>To Do List</h1>
  <!-- 
  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  -->
  

  <div class="row">
    {#if file == null}
      <Home on:submit={handleCreateOrOpenSubmit}/>
    {:else}
      <Edit file={file} filePath={filePath}/>
    {/if}
  </div>

</main>

<!--
<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
-->