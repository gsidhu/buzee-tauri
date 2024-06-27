<script lang="ts">
  import "../app.css";
  import { onMount, onDestroy } from "svelte";
  import StatusBar from "../layout/StatusBar.svelte";
  import * as Dialog from "$lib/components/ui/dialog";

  // import { goto } from '$app/navigation';
  import KeyboardListeners from "$lib/utils/keyboardListeners.svelte";

  import EventListeners from "$lib/utils/eventListeners.svelte";
  import { trackEvent } from "@aptabase/web";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  // import { Command } from '@tauri-apps/plugin-shell';
  import { isMac, windowBlurred, cronJobSet, userPreferences, disableInteraction } from "$lib/stores";

  import { check } from "@tauri-apps/plugin-updater";
  import { ask } from "@tauri-apps/plugin-dialog";
  var appMode: string = "menubar";

  async function maximiseWindow() {
    console.log("double click");
    await window.electronAPI.maximiseWindow();
}

  async function startSerialEventListener() {
    await listen<Payload>("event-name", (event: any) => {
        console.log("Event triggered from rust!\nPayload: " + event.payload.message);
    });
  }

  async function checkForAppUpdates() {
    // const update = { version: "v1.0.0", body: "buzee"};
    const update = await check();

    if (update?.available) {
        const yes = await ask(
            `Update to v${update.version} is available!\n\nRelease notes: ${update.body}`,
            {
                title: "Update Available",
                kind: "info",
                okLabel: "Update",
                cancelLabel: "Cancel"
            }
        );

        if (yes) {
            trackEvent("click:update_button_click");
            await update.downloadAndInstall();
            await invoke("polite_restart");
        }
    }
  }

  $: if ($disableInteraction === true) {
    console.log("disabling");
    document.body.classList.add("disable-interaction");
    document.getElementById("message-modal-trigger")?.click();
  }

  $: if ($disableInteraction === false) {
    console.log("enabling");
    document.body.classList.remove("disable-interaction");
  }

  onMount(async () => {
    // startSerialEventListener();
    invoke("get_os").then(res => {
        // @ts-ignore
        if (res == "macos") {
            $isMac = true;
        } else {
            $isMac = false;
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
      // document.addEventListener("contextmenu", e => { e.preventDefault(); });
    }

    // setup cron job
    if (!cronJobSet) {
        invoke("setup_cron_job").then(res => {
            console.log("setting up cron job");
            $cronJobSet = true;
        });
    }

    // check for app updates
    checkForAppUpdates();
});
</script>

<KeyboardListeners />
<EventListeners />
<main class={`min-vh-100 main-container ${$windowBlurred ? "grayscale" : ""}`}>
	<!-- <button on:click={() => testFn()}>Test</button> -->
	<slot />
	<StatusBar />
</main>

<!-- <button type="button" id="message-modal-trigger" class="d-none" data-bs-toggle="modal" data-bs-target="#message-modal"></button> -->
 
<Dialog.Root>
  <Dialog.Trigger></Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Updating the Database</Dialog.Title>
      <Dialog.Description></Dialog.Description>
    </Dialog.Header>
    <p>
      {#if $disableInteraction}
        Please wait for a few moments while Buzee updates its database...
      {:else}
        All done! You can now close this message.
      {/if}
    </p>
  </Dialog.Content>
</Dialog.Root>

<style>
	.main-container {
		overflow: hidden !important;
	}
</style>