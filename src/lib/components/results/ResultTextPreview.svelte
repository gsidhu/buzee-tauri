<script lang="ts">
  import { onMount } from "svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { Button } from "$lib/components/ui/button";
  import { save } from '@tauri-apps/plugin-dialog';
  import { selectedResult, selectedResultText, showResultTextPreview } from '$lib/stores';
  import { openFileFolder } from '$lib/utils/searchItemUtils';
 
  export let open = false;
  let isDesktop = true;

  $: $showResultTextPreview = open;

  import { invoke } from '@tauri-apps/api/core';
  import { trackEvent } from '@aptabase/web';
	import { LoaderCircle } from "lucide-svelte";

  async function showTextPreview() {
    trackEvent('showText');
    $selectedResultText = [];
    if ($selectedResult !== undefined) {
      invoke('get_text_for_file', { documentId: $selectedResult.id }).then((res) => {
        // @ts-ignore
        res.forEach(element => {
          $selectedResultText.push(element);
        });
      })
    }
    return true;
  }

  saveAsText
  async function saveAsText() {
    const filePath = await save({
      filters: [{
        name: 'Text Files',
        extensions: ['txt']
      }]
    });
    if (filePath) {
      await invoke("write_text_to_file", { filePath, text: $selectedResultText.join("") });
      await invoke("open_folder_containing_file", { filePath });
    }
  }

  onMount(() => {
    if (window.innerWidth < 1024) {
      isDesktop = false;
    }
    showTextPreview();
  });
</script>

{#if isDesktop}
  <Dialog.Root bind:open>
    <Dialog.Content class="">
      <Dialog.Header>
        <Dialog.Title>
          <button class="text-lg hover:underline hover:cursor-pointer" on:click={() => openFileFolder($selectedResult.path)}>
            {$selectedResult.name}
          </button>
        </Dialog.Title>
        <Dialog.Description>
          <div class="flex justify-between items-center">
            <span>The text below is extracted from the file</span>
            <Button variant="link" on:click={() => saveAsText()}>Save as text</Button>
          </div>
        </Dialog.Description>
      </Dialog.Header>
      <div class="p-4 max-h-[60vh] max-w-[60vw] overflow-auto text-interaction rounded-lg border border-dashed">
        {#if $selectedResultText.length === 0}
          <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
        {:else}
          {#each $selectedResultText as para}
            {para}
          {/each}
        {/if}
      </div>
    </Dialog.Content>
  </Dialog.Root>
{:else}
  <Drawer.Root bind:open>
    <Drawer.Content>
      <Drawer.Header class="text-left">
        <Drawer.Title>
          <button class="text-lg hover:underline hover:cursor-pointer" on:click={() => openFileFolder($selectedResult.path)}>
            {$selectedResult.name}
          </button>
        </Drawer.Title>
        <Drawer.Description>
          <div class="flex justify-between items-center">
            <span>The text below is extracted from the file</span>
            <Button variant="link" on:click={() => saveAsText()}>Save as text</Button>
          </div>
        </Drawer.Description>
      </Drawer.Header>
      <div class="p-4 overflow-auto max-h-[70vh] text-interaction border-t border-dashed">
        {#each $selectedResultText as para}
          {para}
        {/each}
      </div>
    </Drawer.Content>
  </Drawer.Root>
{/if}