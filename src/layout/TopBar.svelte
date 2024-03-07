<script lang="ts">
	import SearchBar from "$lib/components/search/searchBar.svelte";
	import SettingsButton from "./SettingsButton.svelte";
  import { onMount } from 'svelte';
  import { page } from '$app/stores';

  var onSearchPage: boolean = false;
  var appMode: string = "menubar";
  var isMac: boolean = false;

  onMount(async () => {
    // isMac = await window.constants?.isMac();
    // appMode = await window.electronAPI?.getAppMode();
    isMac = true;
    appMode = "window";
		// add an event listener to every time the page changes
		page.subscribe((value) => {
			const route = value.url.pathname;
			if (route) {
				if (route === '/search') {
          onSearchPage = true;
        } else {
          onSearchPage = false;
        }
			}
		});
	});
</script>

<div class={`d-flex flex-row justify-content-between align-items-center gap-4 
        px-3 ${isMac && appMode==="window" ? "pb-2 pt-45 drag" : "py-2"}
    `}>
  {#if onSearchPage}
    <SearchBar />
  {/if}
  <SettingsButton />
</div>

<style>
  .pt-45 {
    padding-top: 2rem !important;
  }
</style>