<script lang="ts">
  export let handleClick = () => {};
  export let animate = true;
	export let icon = '';
	export let id = '';
	export let label = 'Click here';
	export let marginClass = 'mx-2 p-2';
	export let showIcon = false;
	export let showText = true;
	export let showSpinner = false;
	export let title = '';
  export let dataBSToggle = '';
  export let dataBSTarget = '';
	export let type = 'confetti-button';
	export let isDisabled = false;

  function animateButton(e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement; }) {
    e.preventDefault;
    if (animate) {
      //reset animation
      (e.target as HTMLElement)?.classList.remove('animate');
      // add animation class
      (e.target as HTMLElement)?.classList.add('animate');
      setTimeout(function(){
        (e.target as HTMLElement)?.classList.remove('animate');
      },700);
    }
    // when the animation is finished, do whatever the real action is
    setTimeout(() => {
      handleClick();
    }, 700);
  };
</script>

<button
	{id}
	class={`${marginClass} ${type} rounded-md`}
	on:click={(e) => animateButton(e)}
	{title}
  data-bs-toggle={dataBSToggle}
  data-bs-target={dataBSTarget}
	disabled={isDisabled}
>
	{#if showSpinner}
		<div class="spinner-border mx-2" style="height: 1rem; width: 1rem;" role="status">
			<span class="visually-hidden">Loading...</span>
		</div>
	{/if}
	{#if showIcon}
		<i class={`bi ${icon} ${showText ? 'margin-right-75' : ''}`} />
	{/if}
	{#if showText}
		{label}
	{/if}
</button>

<style>
  .confetti-button {
    background-color: var(--purple);
    color: #fff !important;
    cursor: pointer;
    position: relative;
    border: none !important;
    transition: transform ease-in 0.1s, box-shadow ease-in 0.25s;
    box-shadow: 0 2px 25px var(--light-purple);
  }

  .confetti-button:hover {
    transform: translateY(-0.25em);
    box-shadow: 0 2px 25px var(--light-purple);
  }

  .confetti-button:focus { outline: 0; }

  .confetti-button:before, .confetti-button:after {
    position: absolute;
    content: '';
    display: block;
    width: 140%;
    height: 100%;
    left: -20%;
    z-index: -1000;
    transition: all ease-in-out 0.5s;
    background-repeat: no-repeat;
  }

  .confetti-button:before {
    display: none;
    top: -75%;
    background-image: radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, transparent 20%, var(--purple) 20%, transparent 30%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, transparent 10%, var(--purple) 15%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%);
    background-size: 10% 10%, 20% 20%, 15% 15%, 20% 20%, 18% 18%, 10% 10%, 15% 15%, 10% 10%, 18% 18%;
  }

  .confetti-button:after {
    display: none;
    bottom: -75%;
    background-image: radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, transparent 10%, var(--purple) 15%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%), radial-gradient(circle, var(--purple) 20%, transparent 20%);
    background-size: 15% 15%, 20% 20%, 18% 18%, 20% 20%, 15% 15%, 10% 10%, 20% 20%;
  }

  .confetti-button:active {
    transform: scale(0.9);
    background-color: var(--light-purple);
    box-shadow: 0 2px 25px var(--light-purple);
  }

  .confetti-button.animate:before {
    display: block;
    animation: topBubbles ease-in-out 0.75s forwards;
  }

  .confetti-button.animate:after {
    display: block;
    animation: bottomBubbles ease-in-out 0.75s forwards;
  }

  @keyframes topBubbles {  
    0% {
      background-position: 5% 90%, 10% 90%, 10% 90%, 15% 90%, 25% 90%, 25% 90%, 40% 90%, 55% 90%, 70% 90%;
    }
    50% {
      background-position: 0% 80%, 0% 20%, 10% 40%, 20% 0%, 30% 30%, 22% 50%, 50% 50%, 65% 20%, 90% 30%;
    }
    100% {
      background-position: 0% 70%, 0% 10%, 10% 30%, 20% -10%, 30% 20%, 22% 40%, 50% 40%, 65% 10%, 90% 20%;
      background-size: 0% 0%, 0% 0%, 0% 0%, 0% 0%, 0% 0%, 0% 0%;
    }
  }
  @keyframes bottomBubbles {
    0% {
      background-position: 10% -10%, 30% 10%, 55% -10%, 70% -10%, 85% -10%, 70% -10%, 70% 0%;
    }
    50% {
      background-position: 0% 80%, 20% 80%, 45% 60%, 60% 100%, 75% 70%, 95% 60%, 105% 0%;
    }
    100% {
      background-position: 0% 90%, 20% 90%, 45% 70%, 60% 110%, 75% 80%, 95% 70%, 110% 10%;
      background-size: 0% 0%, 0% 0%, 0% 0%, 0% 0%, 0% 0%, 0% 0%;
    }
  }
</style>