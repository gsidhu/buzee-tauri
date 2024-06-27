<script lang="ts">
	import SearchBar from "$lib/components/search/searchBar.svelte";
	import SearchFilters from "$lib/components/search/SearchFilters.svelte";
	import SettingsButton from "./SettingsButton.svelte";
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from "@tauri-apps/api/core";

  var onSearchPage: boolean = false;
  var appMode: string = "menubar";

  onMount(async () => {
    // appMode = await window.electronAPI?.getAppMode();
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

<div class="flex flex-col">
  <div class={`flex flex-row justify-between items-center gap-4 px-3 py-2`}>
    {#if onSearchPage}
      <SearchBar />
    {/if}
    <SettingsButton />
  </div>
  
  {#if onSearchPage}
  <!-- Search Filters -->
  <SearchFilters />
  {/if}
</div>

<style>
  .pt-45 {
    padding-top: 2rem !important;
  }
</style>