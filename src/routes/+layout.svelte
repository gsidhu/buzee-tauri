<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import KeyboardListeners from "$lib/utils/keyboardListeners.svelte";
  import EventListeners from '$lib/utils/eventListeners.svelte';
  import { sendEvent } from '../utils/firebase';
  import { invoke } from "@tauri-apps/api/core";
	import { listen } from '@tauri-apps/api/event';
	import { Command } from '@tauri-apps/plugin-shell';
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

	function testFn() {
		invoke("set_global_shortcut_flag", { flag: true }).then((res) => {
			console.log(res);
		});

		// invoke("set_new_global_shortcut", { newShortcutString: "Alt+Shift+Space" }).then((res) => {
		// 	console.log(res);
		// });

		// invoke("enable_sidecar").then((res) => {
		// 	console.log("enabled sidecar");
		// });
		// const command = Command.sidecar('binaries/textra', [
		// 	'/Users/thatgurjot/Desktop/Gurjot_Arc.png',
		// 	'-o',
		// 	'/Users/thatgurjot/Desktop/output.txt',
		// ])
		// command.execute().then((res) => {
		// 	console.log(res);
		// });
	}

	onMount(async () => {
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

		// Grayscale contents when window blurs
		if (window) {
			window.addEventListener("focus", () => {
				$windowBlurred = false;
			});
			window.addEventListener("blur", () => {
				$windowBlurred = true;
			});
		}
		// Disable right click context menu
		if (document) {
			// document.oncontextmenu = function() { return false; }
		}

		// setup cron job
		invoke("setup_cron_job").then((res) => {
			console.log("setting up cron job");
		});
  });

	onDestroy(() => {
		// unlistenBlurred();
		// unlistenFocussed();
	});
</script>

<KeyboardListeners />
<EventListeners />
<main class={`min-vh-100 main-container ${$windowBlurred ? "grayscale" : ""}`}>
	<button on:click={() => testFn()}>Test</button>
	<slot />
</main>

<style>
	.main-container {
		overflow: hidden !important;
	}
</style>
