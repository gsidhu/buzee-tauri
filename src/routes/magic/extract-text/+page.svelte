<script lang="ts">
  import { fade } from "svelte/transition";
  import TopBar from "../../../layout/TopBar.svelte";
	import ConfettiButton from "$lib/components/ui/confettiButton.svelte";
  import { open, save } from '@tauri-apps/plugin-dialog';
  import { invoke } from "@tauri-apps/api/core";

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
    const text = extractedText.join("\n\n");
    await navigator.clipboard.writeText(text);
  }

  async function downloadTextFile() {
    const text = extractedText.join("\n\n");
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
</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-100">
    <TopBar />
  </div>
  <div class="d-flex flex-column col-10 col-sm-8 mx-auto mb-5">
    <div class="text-center gap-2">
      <div class="page-icon">
        <i class="bi bi-body-text"></i>
      </div>
      <h3>Extract Text from PDF or Image</h3>
      <p>Select a PDF or image file</p>
      <ConfettiButton 
        label="Select File" 
        icon="bi-file-earmark-arrow-up" 
        marginClass="mx-2 my-2"
        showIcon={!extractingOngoing}
        showText={!extractingOngoing}
        showSpinner={extractingOngoing}
        handleClick={() => fileSelectorDialog() } />
    </div>

    {#if extractedText.length > 0}
    <div class="my-4">
      <div class="d-flex justify-content-between">
        <h6>{fileName}</h6>
        <div id="toolbar" class="gap-1 d-flex justify-content-center">
          <button class="btn btn-sm" title="Copy text" on:click={() => copyTextToClipboard()}>
            <i class="bi bi-copy"></i>
          </button>
          <button class="btn btn-sm" title="Save text as a file" on:click={() => downloadTextFile()}>
            <i class="bi bi-download"></i>
          </button>
        </div>
      </div>
      <div id="parent-textbox" class="border border-2 border-light p-2 text-interaction">
        {#each extractedText as paragraph }
          <p>{paragraph}</p>
        {/each}
      </div>
    </div>
    {/if}
  </div>
</div>

<style lang="scss">
  #toolbar > button:hover {
    color: var(--purple);
  }
</style>