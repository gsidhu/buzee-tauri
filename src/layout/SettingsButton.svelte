<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
	import { sendEventToFirebase } from '../utils/firebase';
	import { goto } from '$app/navigation';
  import { onSearchPage } from '$lib/stores';

  function openSettingsMenu() {
    sendEventToFirebase("click:settings_button_click");
    goto('/settings');
    // window.menuAPI?.showSettingsMenu();
  }

  onMount(() => {
		// add an event listener to every time the page changes
		page.subscribe((value) => {
			const route = value.url.pathname;
			if (route) {
				if (route === '/search') {
          $onSearchPage = true;
        } else {
          $onSearchPage = false;
        }
			}
		});
	});

</script>

<div id="settings-button-div" class="no-drag">
  {#if $onSearchPage}
    <button class="btn link-dark" title="Open Settings" on:click={() => openSettingsMenu()}>
      <i id="settings-button" class="bi bi-gear" aria-label="Settings" aria-hidden="true"/>
    </button>
  {:else}
    <a href="/search" id="search-button" class="d-block link-dark" title="Back to Search">
      <i class="bi bi-arrow-left-short" aria-label="Back"></i>
      <i class="bi bi-search" aria-label="Search"/>
    </a>
  {/if}
</div>

<style>
  .btn:active {
    border-color: transparent;
  }
  .btn:hover,
  .btn:focus {
    border-color: transparent;
    transform: rotate(60deg);
    color: var(--hot-pink) !important;
  }

  #search-button:hover {
    transform: scale(1.1);
    color: var(--hot-pink) !important;
  }

  a {
    text-decoration: none !important;
  }
</style>