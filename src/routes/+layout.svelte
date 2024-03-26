<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import KeyboardListeners from "$lib/utils/keyboardListeners.svelte";
  import EventListeners from '$lib/utils/eventListeners.svelte';
  import { sendEvent } from '../utils/firebase';
  import { invoke } from "@tauri-apps/api/core";
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { windowBlurred } from '$lib/stores';

	var appMode: string = "menubar";
  var isMac: boolean = false;

	async function maximiseWindow() {
		console.log('double click');
		await window.electronAPI.maximiseWindow();
	}

  async function startSerialEventListener() {
		await listen<Payload>('event-name', (event: any) => {
			console.log("Event triggered from rust!\nPayload: " + event.payload.message);
		});
	}
	
	let unlistenBlurred: UnlistenFn;
	let unlistenFocussed: UnlistenFn;
	onMount(async () => {
		// Initiate the bg process that tracks window focus
		await invoke("track_window_focus");
		// Grayscale contents when window blurs
		unlistenBlurred = await listen<Payload>('window-blurred', (event: any) => {
			$windowBlurred = true;
		});
		// Remove grayscale when window is in focus
		unlistenFocussed = await listen<Payload>('window-focussed', (event: any) => {
			$windowBlurred = false;
		});
    // startSerialEventListener();
		invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				isMac = true;
			} else {
				isMac = false;
			}
		});
    appMode = "window";
  });

	onDestroy(() => {
		unlistenBlurred();
		unlistenFocussed();
	});
</script>

<KeyboardListeners />
<EventListeners />
<main class={`min-vh-100 main-container ${$windowBlurred ? "grayscale" : ""}`}>
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
