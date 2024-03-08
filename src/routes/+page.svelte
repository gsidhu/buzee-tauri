<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { getDocumentsFromDB } from '$lib/utils/dbUtils';
	import { documentsShown, resultsPerPage } from '$lib/stores';
	import { invoke } from "@tauri-apps/api/core";

	async function getUserPreferences() {
		return await window.settingsAPI?.getUserPreferences();
	}

	function reroute(flag: boolean) {
		if (flag) {
			console.log('go to search');
			goto('/search');
		} else {
			console.log('go to onboarding');
			goto('/onboarding');
		}
	}

	function openFileFolder() {
		invoke("open_file_folder", { filePath: "/Users/thatgurjot/Documents/" }).then((res) => {
			console.log(res);
		});
	}

	function runFileIndexing() {
		invoke("run_file_indexing").then((res) => {
			console.log(res);
		});
	}

	function runSearch() {
		invoke("run_search", { query: "gurjot", page: 0, limit: 10, fileType: "xlsx, pdf" }).then((res) => {
			console.log(res);
		});
	}

	onMount(async () => {
		// Load the first page of documents whenever the app loads
		// TODO: Replace this with pinned documents later
		$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, 'any');

		getUserPreferences().then((res) => {
			// reroute(res.onboardingDone);
			reroute(false);
		});
	});
</script>

<!-- <button class="btn btn-primary" on:click={() => openFileFolder()}>Open File Folder</button>
<button class="btn btn-primary" on:click={() => runFileIndexing()}>Run File Indexing</button>

<button class="btn btn-primary" on:click={() => runSearch()}>Run Search</button> -->