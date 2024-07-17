<script lang="ts">
	import { fade } from 'svelte/transition';
	import PopoverIcon from '$lib/components/ui/popoverIcon.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { trackEvent } from '@aptabase/web';
	import { invoke } from '@tauri-apps/api/core';
	import { isMac, statusMessage, userPreferences, dbCreationInProgress } from '$lib/stores';
	import { check } from '@tauri-apps/plugin-updater';
	import { ask, open, message } from '@tauri-apps/plugin-dialog';
	import * as Dialog from "$lib/components/ui/dialog";
  import Button from "$lib/components/ui/button/button.svelte";
	import * as Card from "$lib/components/ui/card/index.js";
	import { Switch } from "$lib/components/ui/switch";
	import {FilePlus2, ListX, ListChecks} from "lucide-svelte";
	import Separator from '$lib/components/ui/separator/separator.svelte';

	let showSearchSuggestions: boolean;
	let launchAtStartup: boolean;
	let globalShortcutEnabled: boolean;
	let globalShortcut: String;
	let globalShortcutModifiers: String[] = ["Option", ""];
	let globalShortcutCode: String = "Space";
	let automaticBackgroundSyncEnabled: boolean;
	let detailedScanEnabled: boolean;
	let skipParsingPDF: boolean;

	function toggleShowSearchSuggestions() {
		showSearchSuggestions = !showSearchSuggestions;
		$userPreferences.show_search_suggestions = showSearchSuggestions;
		trackEvent('click:toggleShowSearchSuggestions', { showSearchSuggestions });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "show_search_suggestions", value: showSearchSuggestions}).then(() => {
			console.log("Set show search suggestions flag to: " + showSearchSuggestions);
		});
	}

	function toggleLaunchAtStartup() {
		launchAtStartup = !launchAtStartup;
		trackEvent('click:toggleLaunchAtStartup', { launchAtStartup });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "launch_at_startup", value: launchAtStartup}).then(() => {
			console.log("Set launch at startup flag to: " + showSearchSuggestions);
		});
	}

	function toggleGlobalShortcut() {
		globalShortcutEnabled = !globalShortcutEnabled;
		trackEvent('click:toggleGlobalShortcut', { globalShortcutEnabled });
		$statusMessage = `Setting changed. Restarting the app...`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "global_shortcut_enabled", value: globalShortcutEnabled}).then(() => {
			console.log("Set global shortcut flag to: " + globalShortcutEnabled);
		});
	}

	function toggleDetailedScan() {
		detailedScanEnabled = !detailedScanEnabled;
		trackEvent('click:toggleDetailedScan', { detailedScanEnabled });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "detailed_scan", value: detailedScanEnabled}).then(() => {
			console.log("Set detailed scan flag to: " + detailedScanEnabled);
		});
	}

	function toggleSkipParsingPDF() {
		skipParsingPDF = !skipParsingPDF;
		trackEvent('click:toggleSkipParsingPDF', { skipParsingPDF });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "skip_parsing_pdfs", value: skipParsingPDF}).then(() => {
			console.log("Set skipParsingPDF flag to: " + skipParsingPDF);
		});
	}

	function toggleAutomaticBackgroundSync() {
		automaticBackgroundSyncEnabled = !automaticBackgroundSyncEnabled;
		trackEvent('click:toggleAutomaticBackgroundSync', { automaticBackgroundSyncEnabled });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "automatic_background_sync", value: automaticBackgroundSyncEnabled}).then(() => {
			console.log("Set automatic background sync flag to: " + automaticBackgroundSyncEnabled);
		});
	}

	function resetDefault() {
    trackEvent('click:resetDefault');
		$statusMessage = `Settings reset. Restarting the app...`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("reset_user_preferences").then(() => {
			console.log("User preferences reset to default");
		});
	}

	function uninstallApp() {
    trackEvent("click:uninstallApp");
		goto('/uninstall');
	}

	async function addDocsToDB() {
		trackEvent('click:addDocsToDB');
		let isFolder = true;
		const yesFolders = await ask("Would you like to add individual files or complete folders?", { 
			title: 'Files or Folders',
			kind: 'info',
			okLabel: 'Folders',
			cancelLabel: 'Files'
		});
		let filePaths: String[] = [];
		if (yesFolders) {
			let folderPaths = await open({ 
				title: 'Add Folders',
				directory: true,
				recursive: true,
				multiple: true,
				canCreateDirectories: false
			});
			if (folderPaths === null) {
				return;
			}
			filePaths = folderPaths;
		} else {
			let filePathObjects = await open({ 
				title: 'Add Files',
				directory: false,
				multiple: true,
				canCreateDirectories: false
			});
			if (filePathObjects === null) {
				return;
			}
			filePaths = filePathObjects.map((file) => file.path);
			isFolder = false;
		}
		$statusMessage = "Adding documents to the database...";
		$dbCreationInProgress = true;
		invoke("run_file_indexing", {filePaths: filePaths, isFolder: isFolder }).then((res) => {
			console.log(res);
			$statusMessage = "Documents added successfully!";
			setTimeout(() => {
				$statusMessage = "";
				$dbCreationInProgress = false;
			}, 3000);
		});
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
		if ($isMac) {
			globalShortcut = globalShortcut.replace("Alt", "Option");
			globalShortcut = globalShortcut.replace("Super", "Command");
		}
	}

	async function checkForAppUpdates() {
		// const update = { version: "v1.0.0", body: "buzee"};
		const update = await check();
		if (update === null) {
			if ($isMac) {
				await message('Failed to check for updates.\nPlease try again later.', {
					title: 'Error',
					kind: 'error',
					okLabel: 'OK'
				});
				return;
			} else {
				await message('You are on the latest version. Stay awesome!', {
					title: 'No Update Available',
					kind: 'info',
					okLabel: 'OK'
				});
			}
		} else if (update.available) {
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
		} else {
			await message('You are on the latest version. Stay awesome!', {
				title: 'No Update Available',
				kind: 'info',
				okLabel: 'OK'
			});
		}
	}

	onMount(() => {
		invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				$isMac = true;
			} else {
				$isMac = false;
			}
		});

		invoke("get_user_preferences_state").then((res) => {
			console.log(res);
			// @ts-ignore
			$userPreferences = res;
			showSearchSuggestions = $userPreferences.show_search_suggestions;
			launchAtStartup = $userPreferences.launch_at_startup;
			globalShortcutEnabled = $userPreferences.global_shortcut_enabled;
			globalShortcut = $userPreferences.global_shortcut;
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
			if ($isMac) {
				globalShortcut = globalShortcut.replace("Alt", "Option");
				globalShortcut = globalShortcut.replace("Super", "Command");
			}
			console.log(globalShortcut);
			automaticBackgroundSyncEnabled = $userPreferences.automatic_background_sync;
			detailedScanEnabled = $userPreferences.detailed_scan;
			skipParsingPDF = $userPreferences.skip_parsing_pdfs;
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

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Settings</h3>
  <p class="text-sm text-muted-foreground">Tune the knobs to make Buzee yours</p>
</div>
<div class="flex flex-1 flex-col items-center justify-center rounded-lg border border-dashed shadow-sm p-4">
	<table class="w-3/5 table table-bordered mb-4">
		<!-- Buttons / Links -->
		<tr>
			<td class="text-center px-2">
				<button class="btn" on:click={() => addDocsToDB()}>
					<i class="bi bi-plus-circle" />
				</button>
			</td>
			<td class="py-2" role="button" on:click={() => addDocsToDB()}>
				Add Documents
				<div class="flex items-center small-explanation gap-1">
					Add more documents to search in Buzee
					<PopoverIcon
						title="By default, Buzee scans your entire system. You can add files from external drives or network drives here."
					/>
				</div>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<button class="btn" on:click={() => goto('/settings/ignore')}>
					<div class="flex">
						<i class="bi bi-file-earmark-x" />
						<i class="bi bi-folder-x" />
					</div>
				</button>
			</td>
			<td class="py-2" role="button" on:click={() => goto('/settings/ignore')}>
				Ignore List
				<div class="flex items-center small-explanation gap-1">
					<div>List of files and folders that you want Buzee to ignore</div>
				</div>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<button class="btn" on:click={() => goto('/settings/filetype-list')}>
					<div class="flex">
						<i class="bi bi-file-earmark" />
					</div>
				</button>
			</td>
			<td class="py-2" role="button" on:click={() => goto('/settings/filetype-list')}>
				File Type List
				<div class="flex items-center small-explanation gap-1">
					<div>List of file types that Buzee can scan</div>
				</div>
			</td>
		</tr>

		<tr>
			<td colspan="2"><Separator /></td>
		</tr>

		<!-- On/Off Toggles -->
		<tr>
			<td class="text-center px-2">
				<Switch bind:checked={showSearchSuggestions} on:click={() => toggleShowSearchSuggestions()} />
			</td>
			<td class="py-2 skip-hover">
				Show Search Suggestions
				<div class="flex items-center small-explanation gap-1">
					Buzee will suggest search terms from your documents
				</div>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch bind:checked={globalShortcutEnabled} on:click={() => toggleGlobalShortcut()} />
			</td>
			<td class="py-2 skip-hover">
				Allow Global Shortcut
				<div class="flex items-center small-explanation gap-1">
						<div>
							Pressing 
							<Dialog.Root>
								<Dialog.Trigger><code class="small-explanation">{globalShortcut}</code></Dialog.Trigger>
								<Dialog.Content>
									<Dialog.Header>
										<Dialog.Title>Change Global Shortcut</Dialog.Title>
										<Dialog.Description>Changes will take effect on app restart</Dialog.Description>
									</Dialog.Header>
									<div>
										<p>Pressing the global shortcut shows the app from anywhere.</p>
										<p>Current shortcut: <code>{globalShortcut}</code></p>
										<p>Set new shortcut below:</p>
										<div class="flex gap-1">
											<div class="col-4 flex items-center">
												<select bind:value={globalShortcutModifiers[0]}>
													{#if $isMac}
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
											<div class="col-4 flex items-center">
												<select bind:value={globalShortcutModifiers[1]}>
													<option value=""></option>
													{#if $isMac}
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
									<Dialog.Footer>
										<Button
													type="button"
													class="btn btn-success"
													disabled={globalShortcutCode === "" || globalShortcutModifiers[1] === globalShortcutModifiers[0]}
													on:click={() => setNewGlobalShortcut()}>Save</Button>
									</Dialog.Footer>
								</Dialog.Content>
							</Dialog.Root>
							will show the app from anywhere
						</div>
						<PopoverIcon title="Changes will take effect after the app restarts" />
				</div>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch bind:checked={automaticBackgroundSyncEnabled} on:click={() => toggleAutomaticBackgroundSync()} />
			</td>
			<td class="py-2 skip-hover">
				Allow Automatic Background Scan
				<div class="flex items-center small-explanation gap-1">
					<div>This allows Buzee to scan your files automatically (twice an hour)</div>
					<PopoverIcon title="We recommend keeping this setting enabled" />
				</div>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch bind:checked={detailedScanEnabled} on:click={() => toggleDetailedScan()} />
			</td>
			<td class="py-2 skip-hover">
				Scan File Text
				<div class="flex items-center small-explanation gap-1">
					<div>Keep this on so you can search inside files (PDFs are scanned last)</div>
					<PopoverIcon
						title="Disabling this setting may improve speed but reduce quality of search results"
					/>
				</div>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch bind:checked={skipParsingPDF} on:click={() => toggleSkipParsingPDF()} />
			</td>
			<td class="py-2 skip-hover">
				Skip Scanning Text from PDFs and Images
				<div class="flex items-center small-explanation gap-1">
					<div>PDFs and images take time to scan. Disable this if you don't have too many of them.</div>
					<PopoverIcon
						title="Disabling this setting may improve the quality of search results but make the app buggy"
					/>
				</div>
			</td>
		</tr>
	</table>
	<div class="w-3/5 flex justify-between settings-links">
		<div class="relative flex-grow max-w-full flex-1 px-4 text-start mobile-text-center my-1">
			<Button variant="link" class="gap-2 text-xs !px-2" on:click={() => resetDefault()}>
				<span class="font-normal text-xs">Reset Default</span>
				<PopoverIcon title="Reset all settings to default and restart the app" />
			</Button>
		</div>
		<div class="relative flex-grow max-w-full flex-1 px-4 text-end mobile-text-center my-1">
			<Button variant="link" class="gap-2 text-xs !px-2" on:click={() => checkForAppUpdates()}>
				<span class="font-normal text-xs">Check for Updates</span>
			</Button>
		</div>
	</div>
</div>

<style lang="scss">
	.small-explanation {
		font-size: 0.7rem;
		font-weight: 300;
		padding: 0;
		background-color: inherit;

		&:not(code) {
			color: var(--bs-gray);
		}
	}

	.settings-links > div {
		@media (min-width: 576px) {
			font-size: 0.7rem;
			font-weight: 300;
			padding: 0;
			background-color: inherit;
			color: var(--bs-gray);

			.btn {
				font-size: inherit;
			}	
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

	.mobile-text-center {
		@media (max-width: 576px) {
			text-align: center !important;
		}
	}
</style>
