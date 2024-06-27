<script lang="ts">
  import { fade } from "svelte/transition";
  
</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Take a Deep Breath</h3>
  <p class="text-sm text-muted-foreground">Just follow the animation</p>
</div>
<div class="flex flex-1 w-full items-center justify-center rounded-lg border border-dashed shadow-sm">
  <div id="breathing" class="mx-auto mt-5"></div>
</div>

<style>
  #breathing {
		background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
		width: 8rem;
		height: 8rem;
		border-radius: 67% 33% 63% 37% / 46% 32% 68% 54%;
    animation: breathing-circle 10s linear infinite;
	}
</style>