<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  const gradients = [
    ['#fad0c4', '#ffd1ff'],
    ['#12c2e9', '#f64f59'],
    ['#FC5C7D', '#6A82FB'],
    ['#22c1c3', '#fdbb2d'],
    ['#e1eec3', '#f05053'],
    ["#22c1c3", "#fdbb2d"],
    ["#ff9966", "#ff5e62"]
  ]
  const direction = ["top", "right", "bottom", "left"];

  let currentGradient = 0;

  function changeColor() {
    currentGradient += 1;
    currentGradient = currentGradient % gradients.length;

    const breathing = document.getElementById("breathing");
    if (breathing) {
      breathing.style.backgroundImage = `linear-gradient(to ${direction[currentGradient % 4]}, ${gradients[currentGradient][0]} 0%, ${gradients[currentGradient][1]} 100%)`;
    }
  }

  onMount(() => {
    const breathing = document.getElementById("breathing");
    if (breathing) {
      breathing.style.backgroundImage = `linear-gradient(to ${direction[currentGradient % 4]}, ${gradients[currentGradient][0]} 0%, ${gradients[currentGradient][1]} 100%)`;
    }
  });
</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Take a Deep Breath</h3>
  <p class="text-sm text-muted-foreground">Just follow the animation</p>
</div>
<div class="flex flex-1 w-full items-center justify-center rounded-lg border border-dashed shadow-sm">
  <button id="breathing" class="mx-auto mt-5" on:click={() => changeColor()}></button>
</div>

<style>
  #breathing {
		background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
		width: 12rem;
		height: 12rem;
		border-radius: 67% 33% 63% 37% / 46% 32% 68% 54%;
    animation: breathing-circle 10s linear infinite;
	}
</style>