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
          setTimeout(() => {
            $statusMessage = '';
          }, 2000);
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

	onMount(() => {
		invoke("show_ignored_paths").then((res) => {
			// @ts-ignore
			$ignoredPaths = res;
			console.log(ignoredPaths);
		})
	});

</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Ignore List</h3>
  <p class="text-sm text-muted-foreground">Any files or folders that you manually add from the Settings will be automatically removed from this list</p>
</div>
<div class="flex flex-1 flex-col gap-2 items-center justify-between rounded-lg border border-dashed shadow-sm p-4">
	<Dialog.Root>
		<Dialog.Trigger>
			<Button>
				<FolderPlus class="mr-2 h-6 w-6" />
				Add Folder to Ignore List
			</Button>
		</Dialog.Trigger>
		<Dialog.Content>
			<Dialog.Header>
				<Dialog.Title>Add Folder to Ignore List</Dialog.Title>
				<Dialog.Description>Add the full path to the folder</Dialog.Description>
			</Dialog.Header>
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
			<Dialog.Footer>
				<Dialog.Close asChild let:builder>
					<Button variant="secondary" aria-label="Close" builders={[builder]}>Close</Button>
					<Button aria-label="Save" builders={[builder]} disabled={pathToIgnore === ''} on:click={() => addToIgnoreList()}>Save</Button>
				</Dialog.Close>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>

	{#key $ignoredPaths.length }
		<IgnoreList />
	{/key}

		<!-- <button>Remove Item</button>
		<h6>SvelteTable: Ignore Completely</h6>
		<small>Dialog menu to add items</small>

		<h6>SvelteTable: Ignore Text</h6>
		<small>Dialog menu to add items</small>

		<button>Clear List</button>
		<button>Export List (to use in Buzee later)</button>
		<button>Import List (from a previous Buzee installation)</button>
		
		<small>On each save/import, run a formatting check + assign isFolder attribute before passing to the database</small> -->
</div>