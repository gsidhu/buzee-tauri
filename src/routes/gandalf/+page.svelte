<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
  import { fade } from 'svelte/transition';
  import { openInBrowser } from '$lib/utils/miscUtils';
	import Separator from '$lib/components/ui/separator/separator.svelte';
</script>

<div class="page" in:fade>
  <div class="w-full flex justify-center items-center">
    Visit <Button on:click={() => openInBrowser('https://tally.so/r/3E0dAo')} variant="link" class="px-1">https://tally.so/r/3E0dAo</Button>if the survey does not load below
  </div>
  <Separator class="my-2" />
	<iframe data-tally-src="https://tally.so/r/3E0dAo?transparentBackground=1" width="100%" height="100%" frameborder="0" marginheight="0" marginwidth="0" title="Buzee Roadmap Survey"></iframe>
  <script async src="https://tally.so/widgets/embed.js"></script>
</div>

<style>
	.page {
		overflow: hidden !important;
		position: relative;
		width: 100%;
		height: 100%;
	}
</style>
