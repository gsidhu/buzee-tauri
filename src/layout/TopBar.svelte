<script lang="ts">
	import SearchBar from "$lib/components/search/searchBar.svelte";
	import SettingsButton from "./SettingsButton.svelte";
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from "@tauri-apps/api/core";

  var onSearchPage: boolean = false;
  var appMode: string = "menubar";
  var isMac: boolean = false;

  onMount(async () => {
    invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				isMac = true;
			} else {
				isMac = false;
			}
		});
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

<div class={`d-flex flex-row justify-content-between align-items-center gap-4 px-3 py-2`}>
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