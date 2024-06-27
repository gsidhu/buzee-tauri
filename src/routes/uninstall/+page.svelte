<script lang="ts">
  import {fade} from "svelte/transition";
  import TopBar from "../../layout/TopBar.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { isMac } from '$lib/stores';

  function openFolder(path: string) {
    window.electronAPI?.openFileFolder(path);
  }

  onMount(() => {
  });
</script>

<div in:fade={{ delay: 500, duration: 1000 }}>
  
  <div class="flex flex-col w-4/5 pr-4 pl-4 sm:w-2/3 mx-auto mb-5">
    <div class="text-center gap-2">
      <div class="page-icon">
        <i class="bi bi-heartbreak-fill"></i>
      </div>
      <h3>Uninstall Buzee</h3>
      <p>We are sad to see you go.</p>
    </div>
    <p>Here's how you can completely remove Buzee from your computer.</p>
    <ol>
      <!-- svelte-ignore a11y-invalid-attribute -->
      <li><a href="#" tabindex=0 role="button" on:click={() => openFolder('appData')}>Delete the App Data</a>.</li>
    <li>
      {#if $isMac}
        <!-- svelte-ignore a11y-invalid-attribute -->
        Quit the app and delete it from the <a href="#" tabindex=0 role="button" on:click={() => openFolder('uninstall')}>Applications folder</a>.
      {:else}
        <!-- svelte-ignore a11y-invalid-attribute -->
        Quit the App and run the <a href="#" tabindex=0 role="button" on:click={() => openFolder('uninstall')}>Uninstaller</a>.
      {/if}
    </li>
    <div class="mt-5 text-center">
      <p>Thank you for using Buzee. We hope to see you again, soon.</p>
      <a href="https://buzee.co?ref=app" target="_blank">buzee.co</a>
    </div>
  </div>
</div>

<style>
  a {
    color: var(--purple);
  }
</style>