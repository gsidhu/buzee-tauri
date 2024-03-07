<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import SvelteTable from '$lib/components/results/svelteTable.svelte';
	import { documentsShown } from '$lib/stores';

	onMount(async () => {
		if (typeof window !== 'undefined') {
			// await window.electronAPI.disableScroll();
		}
	});

	onDestroy(async () => {
		if (typeof window !== 'undefined') {
			// await window.electronAPI.enableScroll();
		}
	});
</script>

{#key $documentsShown}
	<div class={`results-container mb-3 min-vh-100`}>
		<SvelteTable />
	</div>
{/key}

<style>
	.results-container {
		overflow-x: auto;
		width: 100%;
	}
</style>
