<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import KeyboardListeners from "$lib/utils/keyboardListeners.svelte";
  import EventListeners from '$lib/utils/eventListeners.svelte';
  import { sendEvent } from '../utils/firebase';
  import { invoke } from "@tauri-apps/api/core";
	import { listen } from '@tauri-apps/api/event';

	var appMode: string = "menubar";
  var isMac: boolean = false;
	let windowBlurred: boolean = false;

	async function maximiseWindow() {
		console.log('double click');
		await window.electronAPI.maximiseWindow();
	}

  async function startSerialEventListener() {
		await listen<Payload>('event-name', (event: any) => {
			console.log("Event triggered from rust!\nPayload: " + event.payload.message);
		});
	}

  function buttonClick() {
    invoke("test_app_handle").then((res) => {
      console.log("hi:", res);
    });
  }

	onMount(async () => {
    startSerialEventListener();
		// isMac = await window.constants?.isMac();
    // appMode = await window.electronAPI?.getAppMode();
    isMac = true;
    appMode = "window";
		// // Grayscale contents when window blurs
    // window.electronAPI?.windowBlurred(async () => {
    //   windowBlurred = true;
    // })
    // // Remove grayscale when window is in focus
    // window.electronAPI?.windowFocussed(async () => {
    //   windowBlurred = false;
    // })
		// window.navigationAPI?.showSettingsPage(() => {
    //   sendEvent("goto:settings");
    //   goto("/settings");
    // });
		// window.navigationAPI?.showUninstallPage(() => {
    //   sendEvent("goto:uninstall");
    //   goto("/uninstall");
    // });
		// window.navigationAPI?.showStatsPage(() => {
    //   sendEvent("goto:stats");
    //   goto("/stats");
    // });
		// window.navigationAPI?.showDeepBreathingPage(() => {
    //   sendEvent("goto:deep_breathing");
    //   goto("/deep-breathing");
    // });
		// window.navigationAPI?.showTips(() => {
    //   sendEvent("goto:tips");
    //   goto("/tips");
    // });
  });
</script>

<KeyboardListeners />
<EventListeners />
<main class={`min-vh-100 main-container ${windowBlurred ? "grayscale" : ""}`}>
  <!-- <button on:click={buttonClick}>Test</button> -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
  {#if !isMac && appMode==="window"}
    <div class="w-100 fixed-top drag" style="height: 30px;" on:dblclick={() => maximiseWindow()}></div>
  {/if}
	<slot />
</main>

<style>
	.main-container {
		overflow: hidden !important;
	}
</style>
