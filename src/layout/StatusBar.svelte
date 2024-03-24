<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { pinMode, compactViewMode } from '$lib/stores';
	import {
		documentsShown,
		resultsPerPage,
		searchQuery,
		filetypeShown,
		resultsPageShown,
		searchInProgress,
		dbCreationInProgress
	} from '$lib/stores';
	import { selectAllRows } from '$lib/utils/fileUtils';
	import { getDocumentsFromDB, searchDocuments } from '$lib/utils/dbUtils';
	import { sendEvent } from '../utils/firebase';
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/core';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';

	export let onboardingDone = false;
	let darkMode = false;
	let isMac = false;
	let syncStatus = false;
	let syncCoolingPeriod = false;
	let appMode = 'menubar';
	let numFiles: number = 0;
	let showingResults: boolean = false;
	let dbReady = false;
	const defaultData: Record<string, string> = { component: 'StatusBar' };

	function showStatusBarMenu(option: string) {
		sendEvent('click:showStatusBarMenu', { option, ...defaultData });
		// window.menuAPI?.showStatusBarMenu(option);
		invoke("open_context_menu", {option:"statusbar"}).then((res) => {});
	}

	function reCalculateOnDocsShownChange() {
		numFiles = $documentsShown.length;
		showingResults = numFiles > 20;
		selectAllRows(true); // remove selected class from all rows
	}

	$: $documentsShown && reCalculateOnDocsShownChange();

	function toggleCompactViewMode() {
		$compactViewMode = !$compactViewMode;
		sendEvent('click:toggleCompactViewMode', { $compactViewMode, ...defaultData });
		if ($compactViewMode === true) {
			document.querySelectorAll('td').forEach((el) => {
				el.classList.add('compact-view');
			});
			document.querySelectorAll('th').forEach((el) => {
				el.classList.add('compact-view');
			});
		} else {
			document.querySelectorAll('td').forEach((el) => {
				el.classList.remove('compact-view');
			});
			document.querySelectorAll('th').forEach((el) => {
				el.classList.remove('compact-view');
			});
		}
	}

	async function toggleAppMode() {
		sendEvent('click:toggleAppMode', { ...defaultData, appMode });
		await window.electronAPI?.toggleAppMode();
	}

	async function toggleBackgroundTextProcessing() {
		sendEvent('click:toggleBackgroundTextProcessing', { ...defaultData });
		invoke("run_file_sync");
		// disable `bg-sync-btn` for 10 seconds
		// this allows any pending processes to complete when stopping the sync
		const btn = document.getElementById('bg-sync-btn') as HTMLButtonElement | null;
		if (btn) {
			syncCoolingPeriod = true;
			setTimeout(() => {
				syncCoolingPeriod = false;
			}, 10000);
		}
	}

	function goToSearch() {
		sendEvent('click:goToSearch', { ...defaultData });
		goto('/search?highlight-search-bar=true&q=last%20month');
	}

	function update_files_added_count(filesAddedPayload: Payload) {
		if (filesAddedSpan) {
			if (filesAddedPayload.message == "files_added_complete") {
				$dbCreationInProgress = false;
				dbReady = true;
			}
			filesAddedSpan.innerHTML = filesAddedPayload.data.toString();
		}
	}

	// FOR ONBOARDING PROCESS
	let unlisten_files_added:UnlistenFn;
	let filesAddedSpan: HTMLElement | null;

	$: if ($dbCreationInProgress) {
		filesAddedSpan = document.getElementById('files-added');
	}

	// FOR SYNC STATUS WHEN CLICKED
	let unlisten_sync_status:UnlistenFn;

	onMount(async () => {
		invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				isMac = true;
			} else {
				isMac = false;
			}
		});
		unlisten_files_added = await listen<Payload>('files-added', (event: any) => {
			update_files_added_count(event.payload);
		});
		unlisten_sync_status = await listen<Payload>('sync-status', (event: any) => {
			syncStatus = event.payload.data === 'true';
		});

		// Ask for sync status on each mount to keep it updated in case of page changes
		syncStatus = await invoke("get_sync_status") === 'true';
		
		// on renderer launch
		appMode = "window";
	});

	onDestroy(() => {
		unlisten_files_added();
		unlisten_sync_status();
	});
</script>

<div
	id="status-bar-footer"
	class={`row row-cols-3 mx-0 d-flex flex-row justify-content-between px-2 
      ${showingResults ? 'sticky-bottom' : 'fixed-bottom'}
			${$compactViewMode ? 'compact-view' : ''}
  `}
>
	<!-- Left end -->
	<div class="col px-0 d-flex flex-row justify-content-start" id="status-bar-left">
		{#if onboardingDone}
			Showing {numFiles} {numFiles === 1 ? "file" : "files"}
		{:else if dbReady || $dbCreationInProgress}
			<div>
				{#if $dbCreationInProgress}
					Scanning...
				{:else if dbReady}
					Scan complete!
				{/if}
			</div>
		{:else}
			<div>Hello!</div>
		{/if}
	</div>

	<!-- Center -->
	<div class="col px-0 d-flex flex-row justify-content-center" id="status-bar-center">
		{#if dbReady}
			<button
				type="button"
				class="px-1 mx-1 status-item"
				on:click={() => goToSearch()}
				title="Scan complete. Start searching!"
			>
				<i class="bi bi-check-circle" />
			</button>
		{:else if $searchInProgress || $dbCreationInProgress}
			<div class="d-flex justify-content-center align-items-center">
				<div class="spinner-border spinner-border-sm" role="status">
					<span class="visually-hidden">Loading...</span>
				</div>
			</div>
		{:else if onboardingDone}
			<button
				id="bg-sync-btn"
				type="button"
				class={`px-1 mx-1 status-item ${syncStatus ? (syncCoolingPeriod ? 'disabled-gray' : 'bg-code-pink') : ''}`}
				title={syncCoolingPeriod ? 'Please wait for a few seconds...' : `Background sync is ${syncStatus ? 'running' : 'stopped'}. Click to ${syncStatus ? 'stop' : 'start'}.`}
				on:click={() => toggleBackgroundTextProcessing()}
				disabled={syncCoolingPeriod}
			>
				<i id="bg-sync-icon" class={`bi bi-arrow-repeat ${syncStatus ? 'spin-right' : ''}`} />{` Sync${syncStatus ? 'ing' : ''}`}
			</button>
		{/if}
	</div>

	<!-- Right end -->
	<div class="col px-0 d-flex flex-row justify-content-end" id="status-bar-right">
		{#if onboardingDone}
			<button
				type="button"
				class="px-1 mx-1 status-item"
				title="View the fun stuff"
				on:click={() => showStatusBarMenu('extras')}
			>
				<i class="bi bi-stars" />
			</button>
			{#if $compactViewMode}
				<button
					type="button"
					class="px-1 mx-1 status-item"
					title="Switch to expanded view"
					on:click={() => toggleCompactViewMode()}
				>
					<i class="bi bi-arrows-expand" />
				</button>
			{:else}
				<button
					type="button"
					class="px-1 mx-1 status-item"
					title="Switch to compact view"
					on:click={() => toggleCompactViewMode()}
				>
					<i class="bi bi-arrows-collapse" />
				</button>
			{/if}
		{/if}
		<div class={dbReady || $dbCreationInProgress ? "" : "d-none"}>
			<span id="files-added">0</span> files added
		</div>
	</div>
</div>

<style lang="scss">
	.bg-code-pink {
		background: var(--bs-code-color) !important;
	}
	.disabled-gray {
		background: var(--bs-gray) !important;
		&:hover {
			cursor: not-allowed !important;
		}
	}
	#status-bar-footer.compact-view {
		height: 1.75em;
		line-height: 1.75em;
		font-size: 0.8em;
	}
	#status-bar-footer {
		height: 2em;
		line-height: 2em;
		font-size: 0.85em;
		color: white;
		text-align: center;
		background-color: var(--purple);
		/* background-color: rgb(0, 122, 204); */
		position: fixed;
		bottom: 0px;
		width: 100%;
	}
	.status-item:hover {
		background-color: rgba(255, 255, 255, 0.12);
		cursor: pointer;
	}
	button {
		all: unset;
		cursor: pointer;
	}
</style>
