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
	let globalShortcutModifiers: String[] = ["Option", ""];
	let globalShortcutCode: String = "Space";
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

	function setNewGlobalShortcut() {
		// ensure that globalShortcutModifers[1] is not empty and different from globalShortcutModifiers[0]
		if (globalShortcutModifiers[1] === globalShortcutModifiers[0]) {
			globalShortcutModifiers[1] = "";
		}
		if (globalShortcutModifiers[1] === "") {
			globalShortcut = globalShortcutModifiers[0] + "+" + globalShortcutCode;
		} else {
			globalShortcut = globalShortcutModifiers[0] + "+" + globalShortcutModifiers[1] + "+" + globalShortcutCode;
		}
		console.log(globalShortcut);
		$statusMessage = `Setting changed. Restarting the app...`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_new_global_shortcut", { newShortcutString: globalShortcut }).then((res) => {
			console.log(res);
		});
		if (isMac) {
			globalShortcut = globalShortcut.replace("Alt", "Option");
			globalShortcut = globalShortcut.replace("Super", "Command");
		}
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
			console.log(globalShortcut);
			globalShortcut = globalShortcut.replace("Key", "");
			globalShortcut = globalShortcut.replace("Digit", "");
			if (globalShortcut.split("+").length === 2) {
				globalShortcutModifiers[0] = globalShortcut.split("+")[0];
				globalShortcutModifiers[1] = "";
				globalShortcutCode = globalShortcut.split("+")[1];
			} else if (globalShortcut.split("+").length === 3) {
				globalShortcutModifiers[0] = globalShortcut.split("+")[0];
				globalShortcutModifiers[1] = globalShortcut.split("+")[1];
				globalShortcutCode = globalShortcut.split("+")[2];
			}
			console.log(globalShortcutModifiers);
			console.log(globalShortcutCode);
			if (isMac) {
				globalShortcut = globalShortcut.replace("Alt", "Option");
				globalShortcut = globalShortcut.replace("Super", "Command");
			}
			console.log(globalShortcut);
			automaticBackgroundSyncEnabled = userPreferences.automatic_background_sync;
			detailedScanEnabled = userPreferences.detailed_scan;
		});

		const shortcutInput = document.getElementById('shortcut-input');
		shortcutInput?.addEventListener('keydown', function(event) {
			event.preventDefault(); // Prevent the default action of the keypress
			if (event.key === 'Backspace' || event.key === 'Delete') {
				// if the pressed key is backspace or delete, clear the input field
				(shortcutInput as HTMLInputElement).value = '';
				globalShortcutCode = '';
				return;
			}
			// if event.key is alphanumeric, space or F1-F24, proceed
			console.log("pressed:", event.key);
      if (event.key.match(/^[a-zA-Z0-9]$/) || event.key.match(/^F[1-2]?[0-9]$/) || event.key === ' ') {
				let shortcut = '';
				if (event.key === ' ') shortcut = 'Space';
				else shortcut = event.key.toUpperCase();
				// Update the input field value with the captured shortcut
				(shortcutInput as HTMLInputElement).value = shortcut;
				globalShortcutCode = shortcut;
			}
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
				<td class="py-2">
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
				<td class="py-2">
					Launch at Startup
					<div class="d-flex align-items-center small-explanation gap-1">
						Launch the app automatically when your computer starts
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2"
					><input
						type="checkbox"
						bind:checked={globalShortcutEnabled}
						on:click={() => toggleGlobalShortcut()}
					/>
				</td>
				<td class="py-2">
					Allow Global Shortcut
					<div class="d-flex align-items-center small-explanation gap-1">
						{#if isMac}
							<div>
								Pressing <button
									type="button"
									data-bs-toggle="modal"
									data-bs-target="#global-shortcut-modal"><code>{globalShortcut}</code></button
								> will show the app from anywhere
							</div>
							<PopoverIcon title="Changes will take effect after the app restarts" />
						{:else}
							<div>
								Pressing <button
									type="button"
									data-bs-toggle="modal"
									data-bs-target="#global-shortcut-modal"><code>{globalShortcut}</code></button
								> will show the app from anywhere
							</div>
							<PopoverIcon title="Changes will take effect after the app restarts" />
						{/if}
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2"
					><input
						type="checkbox"
						bind:checked={automaticBackgroundSyncEnabled}
						on:click={() => toggleAutomaticBackgroundSync()}
					/>
				</td>
				<td class="py-2">
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
				<td class="py-2">
					Scan File Contents
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Buzee will scan file contents so you can search inside files</div>
						<PopoverIcon
							title="Disabling this setting may improve speed but reduce quality of search results"
						/>
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

<!-- Global Shortcut Modal -->
<div
	class="modal fade"
	id="global-shortcut-modal"
	tabindex="-1"
	aria-labelledby="globalShortcutModal"
	aria-hidden="true"
>
	<div class="modal-dialog modal-dialog-centered">
		<div class="modal-content">
			<div class="modal-header">
				<h1 class="modal-title fs-6" id="globalShortcutModal">Change Global Shortcut</h1>
				<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
			</div>
			<div class="modal-body">
				<p>Pressing the global shortcut shows the app from anywhere.</p>
				<p>Current shortcut: <code>{globalShortcut}</code></p>
				<p>Set new shortcut below:</p>
				<div class="row row-cols-3">
					<div class="col-4 d-flex align-items-center">
						<select bind:value={globalShortcutModifiers[0]}>
							{#if isMac}
								<option value="Super">Command (⌘)</option>
								<option value="Alt">Option (⌥)</option>
								<option value="Control">Control (^)</option>
							{:else}
								<option value="Control">Control</option>
								<option value="Alt">Alt</option>
							{/if}
							<option value="Shift">Shift</option>
						</select>
					</div>
					<div class="col-4 d-flex align-items-center">
						<select bind:value={globalShortcutModifiers[1]}>
							<option value=""></option>
							{#if isMac}
								<option value="Super">Command (⌘)</option>
								<option value="Alt">Option (⌥)</option>
								<option value="Control">Control (^)</option>
							{:else}
								<option value="Control">Control</option>
								<option value="Alt">Alt</option>
							{/if}
							<option value="Shift">Shift</option>
						</select>
					</div>
					<div class="col-4">
						<input
							type="text"
							id="shortcut-input"
							class={`form-control ${globalShortcutCode === '' ? 'border-danger' : ''}`}
							placeholder="Key"
							bind:value={globalShortcutCode}
						/>
					</div>
				</div>
				{#if globalShortcutCode === ""}
					<small class="text-danger small-explanation">Shortcut value cannot be empty</small>
					{#if globalShortcutModifiers[1] === globalShortcutModifiers[0]}<br/>{/if}
				{/if}
				{#if globalShortcutModifiers[1] === globalShortcutModifiers[0]}
					<small class="text-danger small-explanation">Both modifier keys cannot be the same</small>
				{/if}
			</div>
			<div class="modal-footer d-flex justify-content-between">
				<small class="small-explanation">Changes will take effect on app restart</small>
				<button
					type="button"
					class="btn btn-success"
					data-bs-dismiss="modal"
					aria-label="Save"
					disabled={globalShortcutCode === "" || globalShortcutModifiers[1] === globalShortcutModifiers[0]}
					on:click={() => setNewGlobalShortcut()}>Save</button
				>
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
			cursor: default;
			color: var(--purple);
		}
	}

	i {
		font-size: 1.5rem;
	}

	input[type='checkbox'] {
		cursor: pointer;
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
