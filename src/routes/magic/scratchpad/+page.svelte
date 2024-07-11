<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { save } from '@tauri-apps/plugin-dialog';
  import { invoke } from "@tauri-apps/api/core";
  import { scratchPadText } from '$lib/stores';
  import { getFormattedDate } from '$lib/utils/miscUtils';

  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import {Copy, Download, Eraser } from "lucide-svelte";

  let autosaving = false;
  let lastSavedText = "";
  let lastSaved = new Date().toLocaleTimeString();
  let currentDateTime = getFormattedDate();

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

  async function autosaveToFile() {
    console.log('Autosaving...');
    autosaving = true;
    lastSavedText = $scratchPadText;
    await invoke("write_text_to_file", { filePath: "scratchpad", text: $scratchPadText });
    lastSaved = new Date().toLocaleTimeString();
    setTimeout(() => {
      autosaving = false;
    }, 2000);
  }

  onMount(async () => {
    if (document) {
      document.querySelector('textarea')?.focus();
    }

    $scratchPadText = await invoke("read_text_from_txt_file", { filePath: "scratchpad" });

    setInterval(() => {
      if ($scratchPadText.length > 0 && lastSavedText !== $scratchPadText) {
        autosaveToFile();
      }
      currentDateTime = getFormattedDate();
    }, 10000);
  });
</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Scratch Pad</h3>
  <p class="text-sm text-muted-foreground">Anything you write or paste here is saved until you manually clear it</p>
</div>
<div class="flex flex-1 w-full items-center justify-center">
  <div class="w-full h-full !p-2">
    <div class="flex justify-between">
      <div id="save-status" class="text-sm text-muted-foreground flex flex-col justify-center">
        <span class={`${autosaving ? 'block' : 'hidden'}`}>Autosaving...</span>
        {#if $scratchPadText.length > 0 && !autosaving}
          <span class="text-xs text-muted-foreground">Last saved: {lastSaved}</span>
        {/if}
      </div>
      <div id="toolbar" class="gap-1 flex justify-content-center">
        <Button variant="outline" title="Copy text" on:click={() => copyTextToClipboard()}>
          <Copy class="h-4 w-4" />
        </Button>
        <Button variant="outline" title="Save text as a file" on:click={() => downloadTextFile()}>
          <Download class="h-4 w-4" />
        </Button>
        <Button variant="outline" title="Clear all text" on:click={() => $scratchPadText = ""}>
          <Eraser class="h-4 w-4" />
        </Button>
      </div>
    </div>
    <div class="min-h-[60vh] h-full max-h-[90%] rounded-md border border-input shadow-sm flex flex-col w-full items-center space-y-2 pt-2">
      <span class="text-xs text-muted-foreground">{currentDateTime}</span>
      <Textarea bind:value={$scratchPadText} class="h-full border-none resize-none focus-visible:ring-0 focus-visible:ring-offset-0" placeholder="Type your message here."/>
    </div>
  </div>
</div>

<style lang="scss">
  #toolbar > button:hover {
    color: var(--purple);
  }
</style>