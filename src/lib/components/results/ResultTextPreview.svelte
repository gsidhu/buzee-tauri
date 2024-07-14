<script lang="ts">
  import { onMount } from "svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { selectedResult, selectedResultText, showResultTextPreview } from '$lib/stores';
  import { openFileFolder } from '$lib/utils/searchItemUtils';
 
  export let open = false;
  let isDesktop = true;

  $: $showResultTextPreview = open;

  import { invoke } from '@tauri-apps/api/core';
  import { trackEvent } from '@aptabase/web';

  async function showTextPreview() {
    trackEvent('showText');
    if ($selectedResult !== undefined) {
      invoke('get_text_for_file', { documentId: $selectedResult.id }).then((res) => {
        $selectedResultText = [];
        // @ts-ignore
        res.forEach(element => {
          $selectedResultText.push(element);
        });
      })
    }
    return true;
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
          The text below is extracted from the file
        </Dialog.Description>
      </Dialog.Header>
      <div class="p-4 max-h-[60vh] max-w-[60vw] overflow-auto text-interaction rounded-lg border border-dashed">
        {#each $selectedResultText as para}
          {para}
        {/each}
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
          The text below is extracted from the file
        </Drawer.Description>
      </Drawer.Header>
      <div class="p-4 overflow-auto max-h-[70vh] text-interaction rounded-lg border border-dashed">
        {#each $selectedResultText as para}
          {para}
        {/each}
      </div>
    </Drawer.Content>
  </Drawer.Root>
{/if}