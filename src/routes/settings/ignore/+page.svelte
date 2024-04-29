<script lang="ts">
  import { fade } from 'svelte/transition';
  import TopBar from '../../../layout/TopBar.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import IgnoreList from '$lib/components/settings/ignoreList.svelte';
	import ConfettiButton from '$lib/components/ui/confettiButton.svelte';
  import { disableInteraction, ignoredPaths, syncStatus, statusMessage } from '$lib/stores';
	import { open } from '@tauri-apps/plugin-dialog';

	let pathToIgnore = "";
	let ignoreIndex = true;
	let ignoreContent = true;

	$: if (ignoreIndex) { ignoreContent = true }

	function addToIgnoreList() {
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

<div in:fade={{ delay: 0, duration: 500 }}>
	<div id="topbar-bg" class="w-100">
		<TopBar />
	</div>
  <div
		class="d-flex flex-column gap-3 justify-content-center align-items-center col-10 col-sm-8 mx-auto mb-5"
	>
		<div class="page-icon">
			<i class="bi bi-file-earmark-x" /> <i class="bi bi-folder-x" />
		</div>
		<h3>Ignore List</h3>
    <p class="text-center">Any files or folders that you manually add from the Settings will be automatically removed from this list</p>

		<ConfettiButton label="Add Folder to Ignore List" icon="bi-plus-lg" showIcon={true} animate={false} dataBSToggle="modal" dataBSTarget="#add-folder-to-ignore-list-modal" />

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
</div>

<!-- Add Folder to Ignore List Modal -->
<div
	class="modal fade"
	id="add-folder-to-ignore-list-modal"
	tabindex="-1"
	aria-labelledby="addFolderToIgnoreListModal"
	aria-hidden="true"
>
	<div class="modal-dialog modal-dialog-centered">
		<div class="modal-content">
			<div class="modal-header">
				<h1 class="modal-title fs-6" id="addFolderToIgnoreListModal">Add Folder to Ignore List</h1>
				<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
			</div>
			<div class="modal-body">
				<p>Add the full path to the folder</p>
				<div class="d-flex justify-content-between gap-2 mt-3">
					<input
						type="text"
						id="shortcut-input"
						class="form-control"
						placeholder="/path/to/folder"
						bind:value={pathToIgnore}
					/>
					<button class="btn btn-sm btn-warning" on:click={() => showFolderDialog()}>
						<i class="bi bi-folder" />
					</button>
				</div>
				<div class="d-flex justify-content-between mt-3">
					<div class="form-check">
						<input
							class="form-check-input"
							type="checkbox"
							value=""
							id="ignore-index"
							bind:checked={ignoreIndex}
						/>
						<label class="form-check-label" for="ignore-index">
							Ignore Scanning
						</label>
					</div>
					<div class="form-check">
						<input
							class="form-check-input"
							type="checkbox"
							value=""
							id="ignore-content"
							bind:checked={ignoreContent}
						/>
						<label class="form-check-label" for="ignore-content">
							Ignore Content
						</label>
					</div>
				</div>
			</div>
			<div class="modal-footer d-flex justify-content-between">
				<small class="small-explanation"></small>
				<button
					type="button"
					class="btn btn-success"
					data-bs-dismiss="modal"
					aria-label="Save"
					on:click={() => addToIgnoreList()}>Save</button
				>
			</div>
		</div>
	</div>
</div>