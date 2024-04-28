<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import StatusBar from '../layout/StatusBar.svelte';
	import { goto } from '$app/navigation';
	import KeyboardListeners from "$lib/utils/keyboardListeners.svelte";
  import EventListeners from '$lib/utils/eventListeners.svelte';
  import { sendEventToFirebase } from '../utils/firebase';
  import { invoke } from "@tauri-apps/api/core";
	import { listen } from '@tauri-apps/api/event';
	import { Command } from '@tauri-apps/plugin-shell';
	import { windowBlurred, cronJobSet, userPreferences, disableInteraction } from '$lib/stores';
	import { check } from '@tauri-apps/plugin-updater';
	import { ask, message } from '@tauri-apps/plugin-dialog';

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
		// invoke("get_user_preferences_state").then((res) => {
		// 	console.log(res);
		// });

		// invoke("set_global_shortcut_flag", { flag: true }).then((res) => {
		// 	console.log(res);
		// });

		// invoke("set_new_global_shortcut", { newShortcutString: "Alt+Shift+Space" }).then((res) => {
		// 	console.log(res);
		// });

		invoke("run_sidecar").then((res) => {
			console.log("running or ran sidecar");
		});
		// const command = Command.sidecar('binaries/textra', [
		// 	'/Users/thatgurjot/Desktop/Gurjot_Arc.png',
		// 	'-o',
		// 	'/Users/thatgurjot/Desktop/output.txt',
		// ])
		// command.execute().then((res) => {
		// 	console.log(res);
		// });
	}

	async function checkForAppUpdates() {
		// const update = { version: "v1.0.0", body: "buzee"};
		const update = await check();
		if (update?.available) {
			const yes = await ask(`Update to v${update.version} is available!\n\nRelease notes: ${update.body}`, { 
				title: 'Update Available',
				kind: 'info',
				okLabel: 'Update',
				cancelLabel: 'Cancel'
			});
			if (yes) {
				await update.downloadAndInstall();
				await invoke("polite_restart");
			}
		}
	}

	$: if ($disableInteraction === true) {
		console.log("disabling");
		document.body.classList.add('disable-interaction');
		document.getElementById('message-modal-trigger')?.click();
	}

	$: if ($disableInteraction === false) {
		console.log("enabling");
		document.body.classList.remove('disable-interaction');
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

		// get user preferences
		$userPreferences = await invoke("get_user_preferences_state");

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
		if (!cronJobSet) {
			invoke("setup_cron_job").then((res) => {
				console.log("setting up cron job");
				$cronJobSet = true;
			});
		}
		// check for app updates
		checkForAppUpdates();
  });

	onDestroy(() => {
		// unlistenBlurred();
		// unlistenFocussed();
	});
</script>

<KeyboardListeners />
<EventListeners />
<main class={`min-vh-100 main-container ${$windowBlurred ? "grayscale" : ""}`}>
	<!-- <button on:click={() => testFn()}>Test</button> -->
	<slot />
	<StatusBar />
</main>

<button id="message-modal-trigger" type="button" class="d-none" data-bs-toggle="modal" data-bs-target="#message-modal"></button>

<!-- Message Modal -->
<div
	class="modal fade"
	id="message-modal"
	tabindex="-1"
	aria-labelledby="messageModal"
	aria-hidden="true"
>
	<div class="modal-dialog modal-dialog-centered">
		<div class="modal-content">
			<div class="modal-header">
				<h1 class="modal-title fs-6">Updating the Database</h1>
				<button disabled={$disableInteraction} type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
			</div>
			<div class="modal-body">
				<p>
					{#if $disableInteraction}
						Please wait for a few moments while Buzee updates its database...
					{:else}
						All done! You can now close this message.
					{/if}
				</p>
			</div>
		</div>
	</div>
</div>

<style>
	.main-container {
		overflow: hidden !important;
	}
</style>
