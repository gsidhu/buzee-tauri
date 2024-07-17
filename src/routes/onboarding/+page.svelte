<script lang="ts">
	import { onMount } from 'svelte';
	import Onboarding from '$lib/components/onboarding/onboarding.svelte';
	import { dbCreationInProgress } from '$lib/stores';

	onMount(() => {
	});
</script>

<div class="page relative h-full rounded-sm">
	<div id="gradient"></div>
	{#if $dbCreationInProgress}
		<div id="searching">
			<lottie-player src="/searching-files.json" background="transparent"  speed="1"  style="width: 200px; height: 150px;" loop autoplay></lottie-player>
		</div>
	{/if}
	<div id="breathing" class="d-none"></div>
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

	#gradient {
		background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
		opacity: 0.7;
		position: absolute;
		top: 0;
		right: 0;
		width: 200px;
		height: 200px;
		z-index: -1;
		border-radius: 66% 34% 45% 55% / 8% 61% 39% 92%;
		animation: morph 10s infinite;
	}

	@keyframes morph {
		0% {
			border-radius: 66% 34% 45% 55% / 8% 61% 39% 92%;
			top: 10px;
			right: 10px;
		}
		20% {
			border-radius: 66% 34% 54% 46% / 20% 61% 39% 80%;
			top: 12px;
			right: 8px;
		}
		40% {
			border-radius: 58% 42% 73% 27% / 29% 48% 52% 71%;
			top: 15px;
			right: 5px;
		}
		60% {
			border-radius: 67% 33% 63% 37% / 46% 32% 68% 54%;
			top: 12px;
			right: 9px;
		}
		80% {
			border-radius: 66% 35% 57% 43% / 24% 53% 49% 79%;
			top: 10px;
			right: 13px;
		}
		100% {
			border-radius: 66% 34% 45% 55% / 8% 61% 39% 92%;
			top: 10px;
			right: 10px;
		}
	}

	#breathing {
		background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
		opacity: 0;
		position: absolute;
		top: 30%;
		left: 70%;
		width: 100px;
		height: 100px;
		z-index: -1;
		border-radius: 67% 33% 63% 37% / 46% 32% 68% 54%;
	}
</style>

