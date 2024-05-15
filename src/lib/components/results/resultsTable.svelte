<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import SvelteTable from '$lib/components/results/svelteTable.svelte';
	import IconsGrid from './iconsGrid.svelte';
	import { documentsShown, showIconGrid, preferLastOpened, selectedResult, selectedResultText } from '$lib/stores';

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

{#key $documentsShown || $preferLastOpened}
	<div class={`results-container mb-4 min-vh-100`}>
		{#if $showIconGrid}
			<IconsGrid />
		{:else}
			<SvelteTable />
		{/if}
	</div>

	<!-- File Text Modal -->
	{#if $selectedResult}
		<div
		class="modal fade"
		id="file-text-modal"
		tabindex="-1"
		aria-labelledby="fileTextModal"
		aria-hidden="true"
		>
		<div class="modal-dialog modal-dialog-centered">
			<div class="modal-content">
				<div class="modal-header">
					<h1 class="modal-title fs-6" id="fileTextModal">
						{$selectedResult.name}
					</h1>
					<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
				</div>
				<div class="modal-body text-interaction">
					{#if $selectedResultText.length > 0}
						<p>
							{#each $selectedResultText as para}
								{para}
							{/each}
						</p>
					{:else}
						<h6>Nothing to display here.</h6>
						<p>Either the file does not contain any text or it has not been scanned yet.</p>
					{/if}
				</div>
			</div>
		</div>
		</div>
	{/if}
{/key}

<style>
	.results-container {
		overflow-x: auto;
		width: 100%;
	}

	.modal-dialog-centered {
		max-width: 80vw;
	}
</style>
