<script lang="ts">
	import { onMount } from 'svelte';
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

	export let onboardingDone = false;
	let darkMode = false;
	let isMac = false;
	let syncStatus = false;
	let appMode = 'menubar';
	let numFiles: number = 0;
	let showingResults: boolean = false;
	let dbReady = false;
	const defaultData: Record<string, string> = { component: 'StatusBar' };

	function showStatusBarMenu(option: string) {
		sendEvent('click:showStatusBarMenu', { option, ...defaultData });
		// window.menuAPI?.showStatusBarMenu(option);
		invoke("open_context_menu", {option:"statusbar"}).then((res) => {
      console.log("context:", res);
    });
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
		syncStatus = await window.electronAPI?.toggleBackgroundTextProcessing();
	}

	function goToSearch() {
		sendEvent('click:goToSearch', { ...defaultData });
		goto('/search?highlight-search-bar=true&q=last%20month');
	}

	onMount(async () => {
		// isMac = await window.constants?.isMac();
		isMac = true;
		// window.electronAPI?.setNumDocs(async (numDocs: number) => {
		// 	numFiles = numDocs;
		// 	showingResults = numFiles > 20;
		// 	$resultsPerPage = numDocs;
		// 	$searchInProgress = true;
		// 	if ($searchQuery === '') {
		// 		$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, $filetypeShown.slice(1));
		// 	} else {
		// 		$documentsShown = await searchDocuments(
		// 			$searchQuery,
		// 			$resultsPageShown,
		// 			$resultsPerPage,
		// 			$filetypeShown.slice(1)
		// 		);
		// 	}
		// 	$searchInProgress = false;
		// });
		// window.electronAPI?.setBackgroundTextProcessRunningStatus(async (status: boolean) => {
		// 	syncStatus = status;
		// });
		// Get the sync status on each mount
		// syncStatus = await window.dbAPI?.getBackgroundTextProcessRunningStatus();
		// Set the app mode
		//// on renderer launch
		// appMode = await window.electronAPI?.getAppMode();
		appMode = "window";
		//// on function call (defunct cuz renderer is reloaded on mode change)
		// window.electronAPI?.setAppMode(async (mode: string) => {
		// 	console.log('setAppMode', mode);
		// 	appMode = mode;
		// });
		// Refresh documentsShown when a doc is deleted
		// window.electronAPI?.docDeleted(async (filepath: string) => {
		// 	console.log('docDeleted', filepath);
		// 	$documentsShown.splice(
		// 		$documentsShown.findIndex((doc) => doc.path === filepath),
		// 		1
		// 	);
		// });
		// window.dbAPI?.getDBStats(async (result: DBStat[]) => {
		// 	// set these variables only during onboarding
		// 	page.subscribe((value) => {
		// 	const route = value.url.pathname;
		// 		if (route) {
		// 			if (route === '/onboarding') {
		// 				dbReady = true;
		// 				$dbCreationInProgress = false;
		// 			}
		// 		}
		// 	});
		// });
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
			Showing {numFiles} docs
		{:else if dbReady}
			<div>Scan complete</div>
		{:else if $dbCreationInProgress}
			<div>Scanning...</div>
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
				class={`px-1 mx-1 status-item ${syncStatus ? 'bg-code-pink' : ''}`}
				title={`Background sync is ${syncStatus ? 'running' : 'stopped'}. Click to ${
					syncStatus ? 'stop' : 'start'
				}.`}
				on:click={() => toggleBackgroundTextProcessing()}
			>
				<i id="bg-sync-icon" class={`bi bi-arrow-repeat ${syncStatus ? 'spin-right' : ''}`} />{` Sync${syncStatus ? 'ing' : ''}`}
			</button>
			<!-- Added here for debugging -->
			<!-- <a href="/onboarding" class="link-light">Go to onboarding</a> -->
		{:else}
			<!-- Added here for debugging -->
			<!-- <a href="/search" class="link-light">Go to search</a> -->
		{/if}
	</div>

	<!-- Right end -->
	<div class="col px-0 d-flex flex-row justify-content-end" id="status-bar-right">
		<!-- {#if $pinMode}
      <button
        type="button"
        class="px-1 mx-1 status-item"
        title="View or Set pinned files"
        on:click={() => ($pinMode = !$pinMode)}
        on:contextmenu={() => showStatusBarMenu('pin')}
      >
        <i class="bi bi-pin-fill" />
      </button>
    {:else}
      <button
        type="button"
        class="px-1 mx-1 status-item"
        title="View or Set pinned files"
        on:click={() => ($pinMode = !$pinMode)}
        on:contextmenu={() => showStatusBarMenu('pin')}
      >
        <i class="bi bi-pin" />
      </button>
    {/if}
    <button
      type="button"
      class="px-1 mx-1 status-item"
      title="View the folder containing the database file"
      on:click={() => window.electronAPI.openFileFolder("dbPath")}
    >
      <i class="bi bi-database" />
    </button>
    {#if darkMode}
      <button type="button" class="px-1 mx-1 status-item" on:click={() => (darkMode = !darkMode)}>
        <i class="bi bi-brightness-high" />
      </button>
    {:else}
      <button type="button" class="px-1 mx-1 status-item" on:click={() => (darkMode = !darkMode)}>
        <i class="bi bi-moon-stars" />
      </button>
    {/if} -->
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
		<button
			type="button"
			class="px-1 mx-1 status-item"
			title={`Open the app in ${appMode === 'menubar' ? 'window' : 'menubar'} mode (${isMac ? '⌘' : 'Ctrl'} + Shift + F)`}
			on:click={() => toggleAppMode()}
		>
			<i class={`bi ${appMode === 'menubar' ? 'bi-fullscreen' : 'bi-fullscreen-exit'}`} />
		</button>
	</div>
</div>

<style>
	.bg-code-pink {
		background: var(--bs-code-color) !important;
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
