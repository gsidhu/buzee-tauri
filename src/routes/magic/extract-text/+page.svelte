<script lang="ts">
  import { fade } from "svelte/transition";
	import ConfettiButton from "$lib/components/ui/confettiButton.svelte";
  import { open, save } from '@tauri-apps/plugin-dialog';
  import { invoke } from "@tauri-apps/api/core";
  import Button from "$lib/components/ui/button/button.svelte";
  import {Copy, Download, LoaderCircle } from "lucide-svelte";

  let filePath = "";
  let fileName = "";
  let extractingOngoing = false;
  let extractedText: String[] = [];

  async function fileSelectorDialog() {
    const result = await open({
      directories: false,
      multiple: false,
      filters: [
        {
          name: "PDF Files",
          extensions: ["pdf"],
        },
        {
          name: "Image Files",
          extensions: ["png", "jpg", "jpeg"],
        },
      ],
    });
    if (result && result.path && result.name) {
      filePath = result.path;
      fileName = result.name;
    }
    extractText();
  }

  async function extractText() {
    extractedText = [];
    extractingOngoing = true;
    extractedText = await invoke("extract_text_from_pdf_file", { filePath });
    extractingOngoing = false;
  }

  async function copyTextToClipboard() {
    const text = extractedText.join("\n");
    await navigator.clipboard.writeText(text);
  }

  async function downloadTextFile() {
    const text = extractedText.join("\n");
    const saveFilePath = await save({
      filters: [{
        name: 'Text Files',
        extensions: ['txt']
      }]
    });
    if (saveFilePath) {
      await invoke("write_text_to_file", { filePath: saveFilePath, text });
      await invoke("open_folder_containing_file", { filePath: saveFilePath });
    }
  }

  async function openFileFolder() {
    await invoke("open_folder_containing_file", { filePath });
  }
</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Extract Text from PDF or Image</h3>
  <p class="text-sm text-muted-foreground">Select a PDF or image file to begin extraction</p>
</div>
<div class="flex flex-1 flex-col items-center justify-center rounded-lg border border-dashed shadow-sm">
  <div class="flex flex-col items-center gap-1 text-center">
    <Button class="mt-4" on:click={() => fileSelectorDialog()} disabled={extractingOngoing ? true : false}>
      {#if extractingOngoing}
        <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
      {:else}
        Select File
      {/if}
    </Button>
  </div>

  {#if extractedText.length > 0}
    <div class="my-4 px-4 w-full">
      <div class="flex justify-between">
        <Button variant="link" on:click={() => openFileFolder()}>{fileName}</Button>
        <div id="toolbar" class="gap-1 flex !p-2 justify-content-center">
          <Button variant="outline" title="Copy text" on:click={() => copyTextToClipboard()}>
            <Copy class="h-4 w-4" />
          </Button>
          <Button variant="outline" title="Save text as a file" on:click={() => downloadTextFile()}>
            <Download class="h-4 w-4" />
          </Button>
        </div>
      </div>
      <div id="parent-textbox" class="border border-2 border-gray-100 p-2 text-interaction">
        {#each extractedText as paragraph }
          <p>{paragraph}</p>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
</style>