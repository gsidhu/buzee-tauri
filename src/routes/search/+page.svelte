<script lang="ts">
	import { fade } from 'svelte/transition';
	import ResultsTable from '$lib/components/results/resultsTable.svelte';
	import TopBar from '../../layout/TopBar.svelte';
</script>

<div class="page" in:fade>
	<div id="topbar-bg" class="sticky-top w-full">
		<TopBar />
	</div>
	<ResultsTable />
</div>

<style>
	.page {
		overflow: hidden !important;
		position: relative;
		width: 100%;
		height: 100%;
	}
</style>
