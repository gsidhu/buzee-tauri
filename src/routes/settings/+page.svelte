<script lang="ts">
	import { fade } from 'svelte/transition';
	import TopBar from '../../layout/TopBar.svelte';
	import PopoverIcon from '$lib/components/ui/popoverIcon.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { sendEventToFirebase } from '../../utils/firebase';
	import { invoke } from '@tauri-apps/api/core';
	import { statusMessage } from '$lib/stores';

	let launchAtStartup: boolean;
	let globalShortcutEnabled: boolean;
	let globalShortcut: String;
	let automaticBackgroundSyncEnabled: boolean;
	let detailedScanEnabled: boolean;
	let isMac: boolean = true;
	let isWin: boolean;
	let userPreferences: UserPreferences;

	function toggleLaunchAtStartup() {
		launchAtStartup = !launchAtStartup;
		sendEventToFirebase('click:toggleLaunchAtStartup', { launchAtStartup });
		$statusMessage = `Setting changed. Restarting the app...`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		window.settingsAPI?.toggleLaunchAtStartup();
	}

	function toggleGlobalShortcut() {
		globalShortcutEnabled = !globalShortcutEnabled;
		sendEventToFirebase('click:toggleGlobalShortcut', { globalShortcutEnabled });
		$statusMessage = `Setting changed. Restarting the app...`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "global_shortcut_enabled", value: globalShortcutEnabled}).then(() => {
			console.log("Set global shortcut flag to: " + globalShortcutEnabled);
		});
	}

	function toggleDetailedScan() {
		detailedScanEnabled = !detailedScanEnabled;
		sendEventToFirebase('click:toggleDetailedScan', { detailedScanEnabled });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "detailed_scan", value: detailedScanEnabled}).then(() => {
			console.log("Set detailed scan flag to: " + detailedScanEnabled);
		});
	}

	function toggleAutomaticBackgroundSync() {
		automaticBackgroundSyncEnabled = !automaticBackgroundSyncEnabled;
		sendEventToFirebase('click:toggleAutomaticBackgroundSync', { automaticBackgroundSyncEnabled });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "automatic_background_sync", value: automaticBackgroundSyncEnabled}).then(() => {
			console.log("Set automatic background sync flag to: " + automaticBackgroundSyncEnabled);
		});
	}

	function resetDefault() {
    sendEventToFirebase('click:resetDefault');
		$statusMessage = `Settings reset. Restarting the app...`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("reset_user_preferences").then(() => {
			console.log("User preferences reset to default");
		});
	}

	function uninstallApp() {
    sendEventToFirebase("click:uninstallApp");
		goto('/uninstall');
	}

	function addDocsToDB() {
		sendEventToFirebase('click:addDocsToDB');
		window.dbAPI?.addDocsToDB();
	}

	onMount(() => {
		invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				isMac = true;
			} else {
				isMac = false;
			}
		});

		invoke("get_user_preferences_state").then((res) => {
			console.log(res);
			// @ts-ignore
			userPreferences = res;
			launchAtStartup = userPreferences.launch_at_startup;
			globalShortcutEnabled = userPreferences.global_shortcut_enabled;
			globalShortcut = userPreferences.global_shortcut;
			if (isMac) {
				globalShortcut = globalShortcut.replace("CmdOrCtrl", "Cmd");
				globalShortcut = globalShortcut.replace("Alt", "Option");
			}
			automaticBackgroundSyncEnabled = userPreferences.automatic_background_sync;
			detailedScanEnabled = userPreferences.detailed_scan;
		});
	});
</script>

<div in:fade={{ delay: 0, duration: 500 }}>
	<div id="topbar-bg" class="w-100">
		<TopBar />
	</div>
	<div
		class="d-flex flex-column gap-3 justify-content-center align-items-center col-10 col-sm-8 mx-auto mb-5"
	>
		<div class="page-icon">
			<i class="bi bi-gear" />
		</div>
		<h3>Settings</h3>
		<table class="table table-bordered w-90 mb-0">
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => addDocsToDB()}>
						<i class="bi bi-plus-circle" />
					</button>
				</td>
				<td class="py-2" on:click={() => addDocsToDB()}>
					Add Documents
					<div class="d-flex align-items-center small-explanation gap-1">
						Add more documents to search in Buzee
						<PopoverIcon
							title="By default, Buzee scans your entire system. You can add files from external drives or network drives here."
						/>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2"
					><input
						type="checkbox"
						bind:checked={launchAtStartup}
						on:click={() => toggleLaunchAtStartup()}
					/></td
				>
				<td class="py-2" on:click={() => toggleLaunchAtStartup()}>
					Launch at Startup
					<div class="d-flex align-items-center small-explanation gap-1">
						Launch the app automatically when your computer starts
					</div>
				</td>
			</tr>
			{#if isMac || isWin}
				<tr>
					<td class="text-center px-2"
						><input
							type="checkbox"
							bind:checked={globalShortcutEnabled}
							on:click={() => toggleGlobalShortcut()}
						/>
					</td>
					<td class="py-2" on:click={() => toggleGlobalShortcut()}>
						Allow Global Shortcut
						<div class="d-flex align-items-center small-explanation gap-1">
							{#if isMac}
								<div>Pressing <code>{globalShortcut}</code> will show the app from anywhere</div>
								<PopoverIcon title="Changes will take effect after the app restarts" />
							{:else if isWin}
								<div>Pressing <code>{globalShortcut}</code> will show the app from anywhere</div>
								<PopoverIcon title="Changes will take effect after the app restarts" />
							{/if}
						</div>
					</td>
				</tr>
			{/if}
			<tr>
				<td class="text-center px-2"
					><input
						type="checkbox"
						bind:checked={automaticBackgroundSyncEnabled}
						on:click={() => toggleAutomaticBackgroundSync()}
					/>
				</td>
				<td class="py-2" on:click={() => toggleAutomaticBackgroundSync()}>
					Allow Automatic Background Scan
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Buzee will automatically scan your system in the background twice an hour</div>
						<PopoverIcon title="We recommend keeping this setting enabled" />
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2"
					><input
						type="checkbox"
						bind:checked={detailedScanEnabled}
						on:click={() => toggleDetailedScan()}
					/>
				</td>
				<td class="py-2" on:click={() => toggleDetailedScan()}>
					Scan File Contents
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Buzee will scan file contents so you can search inside files</div>
						<PopoverIcon title="Disabling this setting may improve speed but reduce quality of search results" />
					</div>
				</td>
			</tr>
		</table>
		<div class="d-flex w-90 justify-content-between small-explanation">
			<div>
				<button class="btn btn-sm link-danger px-0" on:click={() => resetDefault()}>
					Reset Default
				</button>
				<PopoverIcon title="Reset all settings to default and restart the app" />
			</div>
			<div>
				<button class="btn btn-sm link-danger px-0" on:click={() => uninstallApp()}
					>Uninstall App</button
				>
				<PopoverIcon title="Delete all data and uninstall the app" />
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	.small-explanation {
		font-size: 0.7rem;
		font-weight: 300;
		padding: 0;
		background-color: inherit;
		color: var(--bs-gray);

		.btn {
			font-size: inherit;
		}
	}

	.link-danger {
		&:hover {
			text-decoration: underline;
		}
	}

	tr {
		&:not(.skip-hover):hover {
			cursor: pointer;
			// background-color: var(--bs-gray-100);
			color: var(--purple);
		}
	}

	i {
		font-size: 1.5rem;
	}

	input[type='checkbox'] {
		appearance: none;
		position: relative;
		display: inline-block;
		background: lightgrey;
		height: 1.65rem;
		width: 2.75rem;
		vertical-align: middle;
		border-radius: 2rem;
		box-shadow: 0px 1px 3px #0003 inset;
		transition: 0.25s linear background;
	}

	input[type='checkbox']::before {
		content: '';
		display: block;
		width: 1.25rem;
		height: 1.25rem;
		background: #fff;
		border-radius: 1.2rem;
		position: absolute;
		top: 0.2rem;
		left: 0.2rem;
		box-shadow: 0px 1px 3px #0003;
		transition: 0.25s linear transform;
		transform: translateX(0rem);
	}

	:checked {
		background: green !important;
	}
	:checked::before {
		transform: translateX(1rem) !important;
	}
	:focus {
		outline: none;
	}
	:focus-visible {
		outline: 2px solid dodgerblue;
		outline-offset: 2px;
	}
</style>
