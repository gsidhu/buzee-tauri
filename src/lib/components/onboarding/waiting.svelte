<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import ConfettiButton from '../ui/confettiButton.svelte';
	import Game from './game.svelte';
  let showGame = false;

  function startGame() {
    showGame = true;
  }

  onMount(() => {
  })
</script>

<div in:fade={{ delay: 200, duration: 1000 }} id="wait-screen">
  {#if !showGame}
    <h3>Getting Things Ready</h3>
    <p>We are processing your documents. Give us a few seconds.</p>
    <p>Or, you could play a mini-game about your documents!</p>
    <div class="my-8 text-center">
      <ConfettiButton label="Play the Game" handleClick={() => startGame()}/>
    </div>
  {:else}
    <Game />
  {/if}
</div>