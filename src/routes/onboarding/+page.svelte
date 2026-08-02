<script lang="ts">
	import { onMount } from 'svelte';
	import Onboarding from '$lib/components/onboarding/onboarding.svelte';
	import { dbCreationInProgress } from '$lib/stores';

	onMount(() => {
	});
</script>

<div class="page relative h-full rounded-sm">
	{#if $dbCreationInProgress}
		<div id="searching">
			<lottie-player src="/searching-files.json" background="transparent"  speed="1"  style="width: 200px; height: 150px;" loop autoplay></lottie-player>
		</div>
	{/if}
	<div class="flex flex-col h-full justify-center items-center gap-3">
		<Onboarding />
	</div>
</div>

<style>
	.page {
		/* background: #ee9ca710;
		background: -webkit-linear-gradient(to right, #ffdde110, #ee9ca710);
		background: linear-gradient(to right, #ffdde110, #ee9ca710); */
		background: #D3CCE330;
		background: -webkit-linear-gradient(to right, #E9E4F030, #D3CCE330);
		background: linear-gradient(to right, #E9E4F030, #D3CCE330);
	}

	#searching {
		position: absolute;
		top: 5%;
		right: 0%;
	}
</style>

