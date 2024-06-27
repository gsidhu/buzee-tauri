<script lang="ts">
  import { fly } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
	import ConfettiButton from '../ui/confettiButton.svelte';
  const dispatch = createEventDispatcher();

  export let question:any;
  let rangeValue = 30;
  
  function sendUpdate() {
    if (question.type === "single-select") {
      const selectedOption = document.querySelector('input[name="question"]:checked') as HTMLInputElement;
      dispatch('update', selectedOption.value);
      return;
    } else if (question.type === "range") {
      dispatch('update', rangeValue);
      return;
    }
  }
</script>
{#key question.question}
<div class="question" in:fly={{ delay: 200, y: 100 }}>
  <h5>{question.question}</h5>
  <p class="small"><em>{question.description}</em></p>
  {#if question.type === "single-select"}
    {#each question.options as option, index}
    <div class="form-check">
      {#if index === 0}
        <input class="form-check-input" type="radio" id={option} value={option} name="question" checked>
      {:else}
        <input class="form-check-input" type="radio" id={option} value={option} name="question">
      {/if}
      <label class="form-check-label" for={option}>{option}</label>
    </div>
    {/each}
  {:else if question.type === "range"}
    <div class="text-center">
      <label for="customRange" class="form-label">{rangeValue}%</label>
      <input type="range" class="form-range" min="0" max="100" step="10" id="customRange" bind:value={rangeValue}>
    </div>
  {/if}
  <div class="text-center my-3">
    <!-- <button class="btn py-1 px-2 leading-tight text-xs" on:click={() => sendUpdate()}>Make Guess</button> -->
    <ConfettiButton label="Make Guess" animate={false} handleClick={() => sendUpdate()} />
  </div>
</div>
{/key}

<style>
  .question {
    margin: 2rem auto;
  }
  .form-check-input:checked {
    background-color: var(--purple);
    border-color: var(--purple);
  }
  .form-check-input:focus {
    border-color: var(--purple);
    box-shadow: 0 0 0 0.25rem #8c6ff724;
  }
  input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    background: var(--purple); /* color of the slider thumb */
  }

  input[type='range']::-moz-range-thumb {
    background: var(--purple); /* color of the slider thumb */
  }
</style>