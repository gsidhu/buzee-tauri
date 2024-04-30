<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import TopBar from "../../../layout/TopBar.svelte";
  import { save } from '@tauri-apps/plugin-dialog';
  import { invoke } from "@tauri-apps/api/core";
  import { scratchPadText } from '$lib/stores';

  async function copyTextToClipboard() {
    await navigator.clipboard.writeText($scratchPadText);
  }

  async function downloadTextFile() {
    const filePath = await save({
      filters: [{
        name: 'Text Files',
        extensions: ['txt']
      }]
    });
    if (filePath) {
      await invoke("write_text_to_file", { filePath, text: $scratchPadText });
      await invoke("open_folder_containing_file", { filePath });
    }
  }

  onMount(() => {
    if (document) {
      document.querySelector('textarea')?.focus();
    }
  });
</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-100">
    <TopBar />
  </div>
  <div class="d-flex flex-column col-10 col-sm-8 mx-auto mb-5">
    <div class="text-center gap-2">
      <div class="page-icon">
        <i class="bi bi-journal-text"></i>
      </div>
      <h3>Scratch Pad</h3>
      <p>Anything you write or paste here stays till you restart the app</p>
    </div>

    <div class="my-4">
      <div class="d-flex justify-content-end">
        <div id="toolbar" class="gap-1 d-flex justify-content-center">
          <button class="btn btn-sm" title="Copy text" on:click={() => copyTextToClipboard()}>
            <i class="bi bi-copy"></i>
          </button>
          <button class="btn btn-sm" title="Save text as a file" on:click={() => downloadTextFile()}>
            <i class="bi bi-download"></i>
          </button>
          <button class="btn btn-sm" title="Clear all text" on:click={() => $scratchPadText = ""}>
            <i class="bi bi-eraser"></i>
          </button>
        </div>
      </div>
      <textarea bind:value={$scratchPadText} class="w-100 border border-2 border-light p-2"></textarea>
    </div>
  </div>
</div>

<style lang="scss">
  textarea {
    height: 30vh;
  }

  #toolbar > button:hover {
    color: var(--purple);
  }
</style>