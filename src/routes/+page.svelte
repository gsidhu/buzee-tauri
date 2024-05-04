<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { getDocumentsFromDB } from '$lib/utils/dbUtils';
	import { documentsShown, resultsPerPage, userPreferences, base64Images } from '$lib/stores';
	import { invoke } from '@tauri-apps/api/core';

	function reroute(flag: boolean) {
		if (flag) {
			console.log('go to search');
			goto('/search');
		} else {
			console.log('go to onboarding');
			goto('/onboarding');
		}
	}

	onMount(async () => {
		// Load the first page of documents whenever the app loads
		// TODO: Replace this with pinned documents later
		$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, 'any');
		$base64Images = {};
		console.log("page:", $userPreferences);
		// get user preferences here because this somehow loads before layout finishes its onMount
		$userPreferences = await invoke("get_user_preferences_state");
		reroute($userPreferences.onboarding_done);
	});
</script>