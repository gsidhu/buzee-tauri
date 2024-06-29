<script lang="ts">
  import "../app.css";
  import { onMount, onDestroy } from "svelte";
  import { page } from '$app/stores';
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import Menu from "lucide-svelte/icons/menu";
  import KeyboardListeners from "$lib/utils/keyboardListeners.svelte";

  import EventListeners from "$lib/utils/eventListeners.svelte";
  import { trackEvent } from "@aptabase/web";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { isMac, windowBlurred, cronJobSet, userPreferences, disableInteraction, pagePath } from "$lib/stores";

  import { check } from "@tauri-apps/plugin-updater";
  import { ask } from "@tauri-apps/plugin-dialog";

	import SearchBar from "$lib/components/search/searchBar.svelte";
	import SidebarMenu from "$lib/components/sidebar/sidebarMenu.svelte";
	import UserDropdown from "$lib/components/header/userDropdown.svelte";
	import SyncStatusButton from "$lib/components/settings/syncStatusButton.svelte";
  
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

    page.subscribe((value) => {
			const route = value.url.pathname;
			if (route) {
				$pagePath = route;
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
<main class={`min-h-screen max-h-screen overflow-auto ${$windowBlurred ? "grayscale-no" : ""}`}>
	<div class="grid min-h-screen max-h-screen w-full md:grid-cols-[25vw_1fr] lg:grid-cols-[20vw_1fr]">
    <div class="hidden max-h-screen overflow-y-auto border-r bg-muted/40 md:block">
      <SidebarMenu />
    </div>
    <div class="flex flex-col">
      <header class="flex max-w-screen items-center gap-4 border-b bg-muted/40 px-4 h-[60px] lg:px-6">
        <Sheet.Root>
          <Sheet.Trigger asChild let:builder>
            <Button
              variant="outline"
              size="icon"
              class="shrink-0 md:hidden"
              builders={[builder]}
            >
              <Menu class="h-5 w-5" />
              <span class="sr-only">Toggle navigation menu</span>
            </Button>
          </Sheet.Trigger>
          <Sheet.Content side="left" class="flex flex-col">
            <SidebarMenu />
          </Sheet.Content>
        </Sheet.Root>
        <div class="w-full">
          <SearchBar />
        </div>
        <!-- <UserDropdown /> -->
        <SyncStatusButton />
      </header>
      <section class="flex max-w-screen overflow-auto flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
        <slot />
      </section>
    </div>
  </div>
</main>
 
<!-- <Dialog.Root>
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
</Dialog.Root> -->

<style>
  section {
    max-height: calc(100vh - 60px);
  }
</style>