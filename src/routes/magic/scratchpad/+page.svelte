<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { save } from '@tauri-apps/plugin-dialog';
  import { invoke } from "@tauri-apps/api/core";
  import { scratchPadText } from '$lib/stores';

  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import {Copy, Download, Eraser } from "lucide-svelte";

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

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Scratch Pad</h3>
  <p class="text-sm text-muted-foreground">Anything you write or paste here stays till you restart the app</p>
</div>
<div class="flex flex-1 w-full items-center justify-center rounded-lg border border-dashed shadow-sm">
  <div class="w-full h-full !p-2">
    <div class="flex justify-end">
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
    <Textarea bind:value={$scratchPadText} class="!mt-2 min-h-[50vh]" placeholder="Type your message here." />
  </div>
</div>

<style lang="scss">
  #toolbar > button:hover {
    color: var(--purple);
  }
</style>