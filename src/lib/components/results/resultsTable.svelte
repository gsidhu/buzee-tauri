<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import SvelteTable from '$lib/components/results/svelteTable.svelte';
	import { documentsShown } from '$lib/stores';

	onMount(async () => {
		if (document) {
			document.body.style.overflow = 'hidden';
		}
	});

	onDestroy(async () => {
		if (document) {
			document.body.style.overflow = 'auto';
		}
	});
</script>

{#key $documentsShown}
	<div class={`results-container mb-4 min-vh-100`}>
		<SvelteTable />
	</div>
{/key}

<style>
	.results-container {
		overflow-x: auto;
		width: 100%;
	}
</style>
