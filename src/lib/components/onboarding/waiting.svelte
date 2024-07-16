<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { trackEvent } from '@aptabase/web';
  import ConfettiButton from '../ui/confettiButton.svelte';
	import Game from './game.svelte';
  let showBreathing = false;
  let showGame = false;
  let reduceTimer: NodeJS.Timeout;
  let increaseTimer: NodeJS.Timeout;
  
  function changeOpacity(element: HTMLElement, mode: 'increase' | 'reduce') {
    //get the style value with getComputedStyle, which is an accurate representation of 
    //all element styling on load.
    var currentOp = Number(getComputedStyle(element).getPropertyValue("opacity"));
    if (mode === 'reduce') {
      currentOp -= 0.1;
      if(currentOp <= 0) clearInterval(reduceTimer);
    } else {
      currentOp += 0.1;
      if(currentOp >= 1) clearInterval(increaseTimer);
    }
    element.style.opacity = String(currentOp);
  }

  function breathingAnimation() {
    const gradient = document.getElementById("gradient") as HTMLElement;
    const breathing = document.getElementById("breathing") as HTMLElement;
    // fade out gradient
    gradient.style.animationPlayState = 'paused';
    reduceTimer = setInterval(() => {
      changeOpacity(gradient, 'reduce')
    }, 120);
    // fade in breathing
    breathing.classList.remove("d-none");
    breathing.style.animation = "fadein 3s forwards"
    setTimeout(() => {
      breathing.style.opacity = "0.9";
      breathing.style.animation = "breathing-circle 10s linear infinite";
    }, 1000)
  }

  function startBreathingAnimation() {
    trackEvent("onboarding:breathing_animation");
    showBreathing = true;
    breathingAnimation();
  }

  function stopBreathingAnimation() {
    const gradient = document.getElementById("gradient") as HTMLElement;
    const breathing = document.getElementById("breathing") as HTMLElement;
    // pause breathing animatino
    breathing.style.animationPlayState = 'paused';
    // fade in gradient
    increaseTimer = setInterval(() => {
      changeOpacity(gradient, 'increase');
    }, 150);
    gradient.style.removeProperty('animation');
  }

  function startGame() {
    stopBreathingAnimation();
    showBreathing = false;
    showGame = true;
  }

  onMount(() => {
  })
</script>

<div in:fade={{ delay: 200, duration: 1000 }} id="wait-screen" class={showBreathing ? 'faded' : ''}>
  {#if !showGame}
    <h3>Getting Things Ready</h3>
    <p>We are processing your documents. Give us a few seconds.</p>
    <p>Meanwhile, why not take a few deep breaths?</p>
    <div class="my-8 text-center">
      <ConfettiButton label={showBreathing ? 'Follow the animation' : 'Take Deep Breaths'} handleClick={() => startBreathingAnimation()} isDisabled={showBreathing}/>
    </div>
    <p>Or, you could play a mini-game about your documents!</p>
    <div class="my-8 text-center">
      <ConfettiButton label="Play the Game" handleClick={() => startGame()}/>
    </div>
  {:else}
    <Game />
  {/if}
</div>

<style lang="scss">
  .faded {
    opacity: 0.8;
  }
</style>