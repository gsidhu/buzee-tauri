<script lang="ts">
  import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import IgnoreList from '$lib/components/settings/ignoreList.svelte';
	import ConfettiButton from '$lib/components/ui/confettiButton.svelte';
  import { disableInteraction, ignoredPaths, syncStatus, statusMessage } from '$lib/stores';
	import { open } from '@tauri-apps/plugin-dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Folder, FolderPlus } from 'lucide-svelte';
	import * as Dialog from "$lib/components/ui/dialog";
	import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import { Label } from "$lib/components/ui/label/index.js";

	let pathToIgnore = "";
	let ignoreIndex = true;
	let ignoreContent = true;
	let dialogOpen = false;

	$: if (ignoreIndex) { ignoreContent = true }

	function addToIgnoreList() {
		if (pathToIgnore === '') return;
		$statusMessage = `Adding to Ignore List... Please wait.`;
		$syncStatus = true;
		$disableInteraction = true;
		if (ignoreIndex) { ignoreContent = true }
		invoke('ignore_file_or_folder', { path: pathToIgnore, isDirectory: true, shouldIgnoreIndexing: ignoreIndex, shouldIgnoreContent: ignoreContent }).then(() => {
			$statusMessage = `Removed!`;
			$syncStatus = false;
			$disableInteraction = false;
			$ignoredPaths  = [...$ignoredPaths, { path: pathToIgnore, is_folder: true, ignore_indexing: ignoreIndex, ignore_content: ignoreContent }];
		});
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
	}

	onMount(() => {
		invoke("show_ignored_paths").then((res) => {
			// @ts-ignore
			$ignoredPaths = res;
		})
	});

</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Ignore List</h3>
  <p class="text-sm text-muted-foreground">Any files or folders that you manually add from the Settings will be automatically removed from this list</p>
</div>
<div class="flex flex-1 flex-col gap-2 items-center justify-between rounded-lg border border-dashed shadow-sm p-4">
	<Dialog.Root bind:open={dialogOpen}>
		<Dialog.Trigger>
			<Button>
				<FolderPlus class="mr-2 h-6 w-6" />
				Add Folder to Ignore List
			</Button>
		</Dialog.Trigger>
		<Dialog.Content>
			<Dialog.Header>
				<Dialog.Title>
					{#if $disableInteraction || $statusMessage === "Removed!"}Removing items from the database
					{:else}Add Folder to Ignore List
					{/if}
				</Dialog.Title>
				<Dialog.Description>
					{#if $disableInteraction || $statusMessage === "Removed!"}Please wait while the process completes..
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
				<div class="flex justify-between mt-3">
					<div class="flex items-center space-x-2">
						<Checkbox bind:checked={ignoreIndex} aria-labelledby="terms-label" id="ignore-index" />
						<Label
							id="ignore-index-label"
							for="ignore-index"
							class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
						>
							Ignore Scanning
						</Label>
					</div>
					<div class="flex items-center space-x-2">
						<Checkbox bind:checked={ignoreContent} aria-labelledby="terms-label" id="ignore-content" />
						<Label
							id="ignore-content-label"
							for="ignore-content"
							class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
						>
							Ignore Content
						</Label>
					</div>
				</div>
			{/if}
			<Dialog.Footer>
				<Dialog.Close asChild let:builder>
					<Button variant="secondary" disabled={$disableInteraction} aria-label="Close" builders={[builder]}>Close</Button>
				</Dialog.Close>
				<Button aria-label="Save" disabled={pathToIgnore === '' || $disableInteraction || $statusMessage === "Removed!"} on:click={() => addToIgnoreList()}>Save</Button>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>

	{#key $ignoredPaths.length }
		<IgnoreList />
	{/key}
</div>