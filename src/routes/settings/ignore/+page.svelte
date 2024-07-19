<script lang="ts">
  import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import IgnoreList from '$lib/components/settings/ignoreList.svelte';
  import { disableInteraction, ignoredPaths, syncStatus, statusMessage } from '$lib/stores';
	import { open } from '@tauri-apps/plugin-dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { FolderPlus } from 'lucide-svelte';
	import IgnoreDialog from '$lib/components/settings/IgnoreDialog.svelte';

	let dialogOpen = false;

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
	<Button on:click={() => {dialogOpen = true;}}>
		<FolderPlus class="mr-2 h-6 w-6" />
		Add Folder to Ignore List
	</Button>

	<IgnoreDialog {dialogOpen} />

	{#key $ignoredPaths.length }
		<IgnoreList />
	{/key}
</div>