<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
  import { disableInteraction, ignoredPaths, syncStatus, statusMessage, ignoreDialogOpen } from '$lib/stores';
	import { open } from '@tauri-apps/plugin-dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Folder, FolderPlus } from 'lucide-svelte';
	import * as Dialog from "$lib/components/ui/dialog";
	import * as RadioGroup from "$lib/components/ui/radio-group/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
	import { triggerSearch } from '$lib/utils/dbUtils';

	export let pathToIgnore = "";
	let radioValue = "ignore-content-only"
	let ignoreIndexing = false;
	export let dialogOpen = false;

	function addToIgnoreList() {
		if (pathToIgnore === '') return;
		$statusMessage = `Adding to Ignore List... Please wait.`;
		$syncStatus = true;
		$disableInteraction = true;
		if (radioValue === "ignore-completely") {ignoreIndexing = true;}
		else { ignoreIndexing = false; }
		invoke('ignore_file_or_folder', { path: pathToIgnore, isDirectory: true, shouldIgnoreIndexing: ignoreIndexing }).then(() => {
			$statusMessage = `Removed!`;
			$syncStatus = false;
			$disableInteraction = false;
			$ignoredPaths  = [...$ignoredPaths, { path: pathToIgnore, is_folder: true, ignore_indexing: ignoreIndexing }];
		});
    triggerSearch();
	}

	async function showFolderDialog() {
		let folderPath = await open({ 
			title: 'Add Folder to Ignore List',
			directory: true,
			recursive: true,
			multiple: false,
			canCreateDirectories: false
		});
		if (folderPath) {
			pathToIgnore = folderPath;
		}
	}

	$: if (!dialogOpen) {
		$disableInteraction = false;
		$statusMessage = "";
    $ignoreDialogOpen = false;
	}

	onMount(() => {
		invoke("show_ignored_paths").then((res) => {
			// @ts-ignore
			$ignoredPaths = res;
		})
	});

</script>

<Dialog.Root bind:open={dialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>
        {#if $disableInteraction || $statusMessage === "Removed!"}Removing items from the database
        {:else}Add Folder to Ignore List
        {/if}
      </Dialog.Title>
      <Dialog.Description>
        {#if $disableInteraction || $statusMessage === "Removed!"}Please wait while the process completes...
        {:else}Add the full path to the folder
        {/if}
      </Dialog.Description>
    </Dialog.Header>
    {#if $disableInteraction || $statusMessage === "Removed!"}
      {#if $statusMessage === "Removed!"}
        <p>Done!</p>
        <div class="flex justify-center items-center">
          <lottie-player src="/checkmark-done.json" background="transparent"  speed="1"  style="width: 200px; height: 200px;" autoplay></lottie-player>
        </div>
      {:else}
        <p>Our agents are working really hard on this issue.</p>
        <div class="flex justify-center items-center">
          <lottie-player src="/cat-loop.json" background="transparent"  speed="1"  style="width: 200px; height: 200px;" loop autoplay></lottie-player>
          <!-- <img alt="Cat bashing its arms on a laptop computer" src="/cat-agent.webp" width="200" height="200" /> -->
        </div>
      {/if}
    {:else}
      <div class="flex justify-between gap-2">
        <input
          type="text"
          id="shortcut-input"
          class="form-control w-full border border-1 rounded p-1"
          placeholder="/path/to/folder"
          bind:value={pathToIgnore}
        />
        <Button variant="secondary" on:click={() => showFolderDialog()}>
          <Folder class="h-4 w-4" />
        </Button>
      </div>
      <div class="mt-3">
        <RadioGroup.Root bind:value={radioValue}>
          <div class="flex items-center space-x-2">
            <RadioGroup.Item value="ignore-content-only" id="ignore-content-only" />
            <Label for="ignore-content-only" class="text-sm font-normal leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Keep in Index but Ignore Content
            </Label>
          </div>
          <div class="flex items-center space-x-2">
            <RadioGroup.Item value="ignore-completely" id="ignore-completely" />
            <Label for="ignore-completely" class="text-sm font-normal leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Ignore Completely
            </Label>
          </div>
        </RadioGroup.Root>
      </div>
    {/if}
    <Dialog.Footer>
      <Button variant="secondary" disabled={$disableInteraction} aria-label="Close" on:click={() => {$ignoreDialogOpen = false;}}>Close</Button>
      <Button aria-label="Save" disabled={pathToIgnore === '' || $disableInteraction || $statusMessage === "Removed!"} on:click={() => addToIgnoreList()}>Save</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>