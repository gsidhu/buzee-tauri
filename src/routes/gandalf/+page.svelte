<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
  import { fade } from 'svelte/transition';
  import { openInBrowser } from '$lib/utils/miscUtils';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { onMount } from 'svelte';
	import { userPreferences } from '$lib/stores';
	import { invoke } from '@tauri-apps/api/core';
	import { trackEvent } from '@aptabase/web';
	import { X } from 'lucide-svelte';

	let roadmap_survey_answered = false;

	function truthyRoadmapSurveyAnswered() {
		roadmap_survey_answered = true;
		$userPreferences.roadmap_survey_answered = true;
		trackEvent('click:truthyRoadmapSurveyAnswered', { roadmap_survey_answered });
		invoke("set_user_preference", {key: "roadmap_survey_answered", value: roadmap_survey_answered}).then(() => {
			console.log("Set roadmap survey answered flag to: " + roadmap_survey_answered);
		});
	}

	onMount(() => {
		roadmap_survey_answered = $userPreferences.roadmap_survey_answered;
	});
</script>

<div class="page rounded-lg border border-dashed shadow-sm p-4" in:fade>
	{#if roadmap_survey_answered === false}
		<div class="w-full flex justify-between items-center" out:fade={{ duration: 500 }}>
			<Button on:click={() => openInBrowser('https://tally.so/r/3E0dAo')} variant="link" class="px-1 font-normal underline">https://tally.so/r/3E0dAo</Button>
			<Button on:click={() => truthyRoadmapSurveyAnswered()} variant="link" class="px-4 font-normal gap-1">
				Don't show the survey again <X class="h-3 w-3" />
			</Button>
		</div>
		<Separator class="my-2" />
		<iframe data-tally-src="https://tally.so/r/3E0dAo?transparentBackground=1" width="100%" height="100%" frameborder="0" marginheight="0" marginwidth="0" title="Buzee Roadmap Survey"></iframe>
		<script async src="https://tally.so/widgets/embed.js"></script>
	{:else}
		<div class="w-full h-full flex justify-center items-center" in:fade={{ delay: 500 }}>
			You can still access the roadmap survey at <Button on:click={() => openInBrowser('https://tally.so/r/3E0dAo')} variant="link" class="px-1 font-normal underline">https://tally.so/r/3E0dAo</Button>
		</div>
		{/if}
</div>

<style>
	.page {
		overflow: hidden !important;
		position: relative;
		width: 100%;
		height: 100%;
	}

	iframe {
		padding-bottom: 4rem;
	}
</style>
