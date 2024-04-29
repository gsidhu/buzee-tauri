<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { compactViewMode, statusMessage, onSearchPage, userPreferences, syncStatus } from '$lib/stores';
	import {
		documentsShown,
		searchInProgress,
		dbCreationInProgress,
		windowBlurred
	} from '$lib/stores';
	import { selectAllRows } from '$lib/utils/fileUtils';
	import { sendEventToFirebase } from '../utils/firebase';
	import { invoke } from '@tauri-apps/api/core';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	
	let darkMode = false;
	let isMac = false;
	let fileSyncFinished = false;
	let syncCoolingPeriod = false;
	let userAskedToDisable = false;
	let appMode = 'menubar';
	let numFiles: number = 0;
	let showingResults: boolean = false;
	let dbReady = false;
	let filesAddedCount = 0;
	const defaultData: Record<string, string> = { component: 'StatusBar' };

	function showStatusBarMenu(option: string) {
		sendEventToFirebase('click:showStatusBarMenu', { option, ...defaultData });
		// invoke("open_context_menu", {option:"statusbar"}).then((res) => {});
		goto("/magic/");
	}

	function reCalculateOnDocsShownChange() {
		numFiles = $documentsShown.length;
		showingResults = numFiles > 20;
		selectAllRows(true); // remove selected class from all rows
	}

	$: $documentsShown && reCalculateOnDocsShownChange();

	function toggleCompactViewMode() {
		$compactViewMode = !$compactViewMode;
		sendEventToFirebase('click:toggleCompactViewMode', { $compactViewMode, ...defaultData });
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

	async function toggleBackgroundTextProcessing() {
		sendEventToFirebase('click:toggleBackgroundTextProcessing', { ...defaultData });
		// if $syncStatus is true, switch_off is true, so we want to stop the sync
		invoke("run_file_sync", {switchOff: $syncStatus, filePaths: []});
		if ($syncStatus) {
			$statusMessage = "Stopping background scan...";
			setTimeout(() => {$statusMessage = "";}, 3000);
			userAskedToDisable = true;
		} else {
			$statusMessage = "Starting background scan...";
			setTimeout(() => {$statusMessage = "";}, 3000);
		}
		// disable `bg-sync-btn` for 5 seconds
		// this allows any pending processes to complete when stopping the sync
		syncCoolingPeriod = true;
		setTimeout(() => {
			// if userAskedToDisable and sync is still running, then keep the cooling period on
			if (userAskedToDisable && $syncStatus) {
				syncCoolingPeriod = true;
			} else {
				syncCoolingPeriod = false;
			}
		}, 5000);
	}

	// if $syncStatus is false, then reset cooling period and userAskedToDisable
	$: if (!$syncStatus) {
		userAskedToDisable = false;
		syncCoolingPeriod = false;
	}

	function goToSearch(from_onboarding: boolean = false) {
		sendEventToFirebase('click:goToSearch', { ...defaultData });
		if (from_onboarding) {
			// start background processing to get file contents
			toggleBackgroundTextProcessing();
			goto('/search?highlight-search-bar=true&q=this%20month');
		} else {
			goto('/search');
		}
	}

	function update_files_added_count(filesAddedPayload: Payload) {
		filesAddedCount = parseInt(filesAddedPayload.data);
		if (filesAddedPayload.message == "files_added_complete") {
			$dbCreationInProgress = false;
			dbReady = true;
		}
	}

	// FOR ONBOARDING PROCESS
	let unlisten_files_added:UnlistenFn;

	// FOR SYNC STATUS WHEN CLICKED
	let unlisten_sync_status:UnlistenFn;
	// FOR FILE SYNC FINISHED
	let unlisten_file_sync_finished:UnlistenFn;

	onMount(async () => {
		invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				isMac = true;
			} else {
				isMac = false;
			}
		});

		invoke("get_user_preferences_state").then((res) => {
			// @ts-ignore
			$userPreferences = res;
		});

		// Listener for when every batch (500) of files gets added to the database
		unlisten_files_added = await listen<Payload>('files-added', (event: any) => {
			update_files_added_count(event.payload);
			if (event.payload.message === "files_added_complete") {
				$userPreferences.onboarding_done = true;
			}
		});
		// Listener for sync status changes from inside the Tokio process in db_sync.rs
		unlisten_sync_status = await listen<Payload>('sync-status', (event: any) => {
			$syncStatus = event.payload.data === 'true';
		});
		// Listener for when the db_sync process is done
		unlisten_file_sync_finished = await listen<Payload>('file-sync-finished', (event: any) => {
			fileSyncFinished = event.payload.data === 'true';
		});

		// Ask for sync status on each mount to keep it updated in case of page changes
		$syncStatus = await invoke("get_sync_status") === 'true';

		// on renderer launch
		appMode = "window";
	});

	onDestroy(() => {
		unlisten_files_added();
		unlisten_sync_status();
		unlisten_file_sync_finished();
	});
</script>

<div
	id="status-bar-footer"
	class={`row row-cols-3 mx-0 d-flex flex-row justify-content-between px-2 
      ${showingResults ? 'sticky-bottom' : 'fixed-bottom'}
			${$compactViewMode ? 'compact-view' : ''}
			${$windowBlurred ? 'grayscale' : ''}
  `}
>
	<!-- Left end -->
	<div class="col px-0 d-flex flex-row justify-content-start disable-select cursor-default" id="status-bar-left">
		{#if $userPreferences.onboarding_done}
			{#if $onSearchPage}
				<!-- <button
					type="button"
					class="px-1 mx-1 status-item"
					on:click={() => goToSearch()}
					title="View search results"
				> -->
				Showing {numFiles} {numFiles === 1 ? "result" : "results"}
				<!-- </button> -->
			{:else}
				<!-- This is used on the Settings page when adding docs manually -->
				{#if $dbCreationInProgress}
					Scanning... {filesAddedCount}	files added
				{/if}
			{/if}
		{:else if dbReady || $dbCreationInProgress}
			{#if $dbCreationInProgress}
				Scanning... {filesAddedCount}	files added
			{:else if dbReady}
				Scan complete!
			{/if}
		{:else}
			Hello!
		{/if}
	</div>

	<!-- Center -->
	<div class="col px-0 d-flex flex-row justify-content-center disable-select cursor-default" id="status-bar-center">
		{#if $userPreferences.onboarding_done}
			{$statusMessage}
		{:else if dbReady}
			<button
				type="button"
				class="px-1 mx-1 status-item"
				on:click={() => goToSearch(true)}
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
		{/if}
	</div>

	<!-- Right end -->
	<div class="col px-0 d-flex flex-row justify-content-end disable-select" id="status-bar-right">
		{#if $userPreferences.onboarding_done}
			<!-- Notifications -->
			<!-- <div class="dropup dropup-center px-0 mx-0 status-item">
				<button
					id="bg-sync-btn"
					type="button"
					class={`status-item px-1  ${$syncStatus ? (syncCoolingPeriod ? 'disabled-gray' : 'bg-code-pink') : ''}`}
					title={syncCoolingPeriod ? 'Please wait for a few seconds...' : `Background scan is ${$syncStatus ? 'running' : 'stopped'}. Click to ${$syncStatus ? 'stop' : 'start'}.`}
					disabled={syncCoolingPeriod}
					data-bs-toggle="dropdown"
					aria-expanded="false"
					style="all: unset;"
				>
					<i id="bg-sync-icon" class='bi bi-bell' />
				</button>
				<ul class="dropdown-menu py-0 px-2" style="cursor: default;">
					<div class="file-stats-header text-center">
						<strong><small>Notifications</small></strong>
					</div>
					<hr class="mb-1 mt-0" />
					<div><small>Files Indexed</small></div>
					<div><small>124</small></div>
				</ul>
			</div> -->
			<button
				id="bg-sync-btn"
				type="button"
				class={`px-2 status-item ${$syncStatus ? (syncCoolingPeriod ? 'disabled-gray' : 'bg-code-pink') : ''}`}
				title={syncCoolingPeriod ? 
				`${userAskedToDisable ? "Shutting down background processes. Please wait." : "Booting up. Please wait for a few seconds."}` 
				: 
				`Background scan is ${$syncStatus ? 'running' : 'stopped'}. Click to ${$syncStatus ? 'stop' : 'start'}.`}
				on:click={() => toggleBackgroundTextProcessing()}
				disabled={syncCoolingPeriod}
			>
				<i id="bg-sync-icon" class={`bi bi-arrow-repeat ${$syncStatus ? 'spin-right' : ''}`} />
			</button>
			<button
				type="button"
				id="status-bar-extras"
				class="px-2 status-item"
				title="View the fun stuff"
				on:click={() => showStatusBarMenu('extras')}
			>
				<i class="bi bi-stars" />
			</button>
			{#if $compactViewMode}
				<button
					type="button"
					class="px-2 status-item"
					title="Show results in normal view"
					on:click={() => toggleCompactViewMode()}
				>
					<i class="bi bi-arrows-expand" />
				</button>
			{:else}
				<button
					type="button"
					class="px-2 status-item"
					title="Show results in compact view"
					on:click={() => toggleCompactViewMode()}
				>
					<i class="bi bi-arrows-collapse" />
				</button>
			{/if}
		{/if}
	</div>
</div>

<style lang="scss">
	.code-pink {
		color: var(--bs-code-color) !important;
	}
	.bg-code-pink {
		background: var(--bs-code-color) !important;
	}
	.disabled-gray {
		background: var(--bs-gray) !important;
		color: white;
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
