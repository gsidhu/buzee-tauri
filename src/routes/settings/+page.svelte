<script lang="ts">
	import { fade } from 'svelte/transition';
	import PopoverIcon from '$lib/components/ui/popoverIcon.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { trackEvent } from '@aptabase/web';
	import { invoke } from '@tauri-apps/api/core';
	import { isMac, statusMessage, userPreferences, dbCreationInProgress, syncStatus } from '$lib/stores';
	import { check } from '@tauri-apps/plugin-updater';
	import { ask, open, message } from '@tauri-apps/plugin-dialog';
	import * as Dialog from "$lib/components/ui/dialog";
  import Button from "$lib/components/ui/button/button.svelte";
	import * as Select from "$lib/components/ui/select";
	import { Switch } from "$lib/components/ui/switch";
	import {PencilLine, TriangleAlert} from "lucide-svelte";
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import Input from '$lib/components/ui/input/input.svelte';

	let showSearchSuggestions: boolean;
	let launchAtStartup: boolean;
	let globalShortcutDialogOpen = false;
	let globalShortcutEnabled: boolean;
	let globalShortcut: String;
	let globalShortcutModifiers: any[] = [{value: "Alt", label: "Option (⌥)"}, {value: "", label: " "}];
	if (!$isMac) { globalShortcutModifiers[0].label = "Alt"; }
	let globalShortcutCode: String = "Space";
	let automaticBackgroundSyncEnabled: boolean;
	let detailedScanEnabled: boolean;
	let skipParsingPDF: boolean;
	let manualSetupMode: boolean;
	let clearIndexDialogOpen = false;

	function setKeydownHandlerOnGlobalShortuctInput(event: KeyboardEvent) {
		console.log("~>>! pressed:", event.key);
		event.preventDefault(); // Prevent the default action of the keypress
		if (event.key === 'Backspace' || event.key === 'Delete') {
			// if the pressed key is backspace or delete, clear the input field
			const shortcutInput = document.getElementById('shortcut-input');
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
			const shortcutInput = document.getElementById('shortcut-input');
			(shortcutInput as HTMLInputElement).value = shortcut;
			globalShortcutCode = shortcut;
		}
	}

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

	function toggleManualSetupMode() {
		manualSetupMode = !manualSetupMode;
		trackEvent('click:toggleManualSetupMode', { manualSetupMode });
		$statusMessage = `Setting changed!`;
		setTimeout(() => {$statusMessage = "";}, 3000);
		invoke("set_user_preference", {key: "manual_setup", value: manualSetupMode}).then(() => {
			console.log("Set manual setup flag to: " + manualSetupMode);
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

	async function clearIndex() {
    trackEvent('click:clearIndex');
		await invoke("clear_index");
		$statusMessage = `Cleared!`;
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
		$syncStatus = true;
		invoke("run_file_indexing", {filePaths: filePaths, isFolder: isFolder }).then((res) => {
			console.log(res);
			$statusMessage = "Documents added successfully!";
			setTimeout(() => {
				$statusMessage = "";
				$dbCreationInProgress = false;
				$syncStatus = false;
			}, 3000);
		});
	}

	function setNewGlobalShortcut() {
		// ensure that globalShortcutModifers[1] is not empty and different from globalShortcutModifiers[0]
		if (globalShortcutModifiers[1].value === globalShortcutModifiers[0].value) {
			globalShortcutModifiers[1] = {value: "", label: ""};
		}
		if (globalShortcutModifiers[1].value === "") {
			globalShortcut = globalShortcutModifiers[0].value + "+" + globalShortcutCode;
		} else {
			globalShortcut = globalShortcutModifiers[0].value + "+" + globalShortcutModifiers[1].value + "+" + globalShortcutCode;
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
				globalShortcutModifiers[0].value = globalShortcut.split("+")[0];
				globalShortcutModifiers[1].value = "";
				globalShortcutModifiers[0].label = globalShortcut.split("+")[0];
				globalShortcutModifiers[1].label = " ";
				globalShortcutCode = globalShortcut.split("+")[1];
			} else if (globalShortcut.split("+").length === 3) {
				globalShortcutModifiers[0].value = globalShortcut.split("+")[0];
				globalShortcutModifiers[1].value = globalShortcut.split("+")[1];
				globalShortcutModifiers[0].label = globalShortcut.split("+")[0];
				globalShortcutModifiers[1].label = globalShortcut.split("+")[1];
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
			manualSetupMode = $userPreferences.manual_setup;
		});
	});
</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Settings</h3>
  <p class="text-sm text-muted-foreground">Tune the knobs to make Buzee yours</p>
</div>
<div class="flex flex-1 flex-col items-center justify-center rounded-lg border border-dashed shadow-sm p-4">
	<table class="w-4/5 md:w-3/5 table table-bordered my-2">
		<!-- Buttons / Links -->
		<tr class="hover:text-violet-500">
			<td class="text-center px-2">
				<button on:click={() => addDocsToDB()}>
					<i class="bi bi-plus-circle" />
				</button>
			</td>
			<td class="py-2" role="button" on:click={() => addDocsToDB()}>
				Add Documents
				<div class="flex items-center small-explanation gap-1">
					Add more documents to search in Buzee
				</div>
			</td>
			<td class="">
				<PopoverIcon
					title="By default, Buzee scans your entire system. You can add files from external drives or network drives here."
				/>
			</td>
		</tr>
		<tr class="hover:text-violet-500">
			<td class="text-center px-2">
				<button on:click={() => goto('/settings/ignore')}>
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
		<tr class="hover:text-violet-500">
			<td class="text-center px-2">
				<button on:click={() => goto('/settings/filetype-list')}>
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

		<tr class="h-10">
			<td colspan="3"><Separator /></td>
		</tr>

		<!-- On/Off Toggles -->
		<tr>
			<td class="text-center px-2">
				<Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={showSearchSuggestions} on:click={() => toggleShowSearchSuggestions()} />
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
				<Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={globalShortcutEnabled} on:click={() => toggleGlobalShortcut()} />
			</td>
			<td class="py-2 skip-hover">
				Allow Global Shortcut
				<div class="flex items-center small-explanation gap-1">
					Pressing <code class="small-explanation">{globalShortcut}</code>
						<Dialog.Root bind:open={globalShortcutDialogOpen}>
							<Dialog.Trigger class="skip-hover border p-1 rounded-full flex justify-center items-center gap-1 hover:border-violet-500 hover:text-violet-500"><PencilLine class="h-3 w-3"/></Dialog.Trigger>
							<Dialog.Content>
								<Dialog.Header>
									<Dialog.Title>Change Global Shortcut</Dialog.Title>
								</Dialog.Header>
								<div>
									<p>Pressing the global shortcut shows the app from anywhere.</p>
									<p>Current shortcut: <code>{globalShortcut}</code></p>
									<p>Set new shortcut below:</p>
									<div class="flex gap-1 justify-center items-center">
										<div class="col-4 flex items-center">
											<Select.Root bind:selected={globalShortcutModifiers[0]}>
												<Select.Trigger class="md:w-[150px]">
													<Select.Value placeholder={globalShortcutModifiers[0]} />
												</Select.Trigger>
												<Select.Content>
													{#if $isMac}
														<Select.Item value="Super">Command (⌘)</Select.Item>
														<Select.Item value="Alt">Option (⌥)</Select.Item>
														<Select.Item value="Control">Control (^)</Select.Item>
													{:else}
														<Select.Item value="Control">Control</Select.Item>
														<Select.Item value="Alt">Alt</Select.Item>
													{/if}
													<Select.Item value="Shift">Shift</Select.Item>
												</Select.Content>
											</Select.Root>
										</div>
										<div class="col-4 flex items-center">
											<Select.Root bind:selected={globalShortcutModifiers[1]}>
												<Select.Trigger class="md:w-[150px]">
													<Select.Value placeholder={globalShortcutModifiers[1]} />
												</Select.Trigger>
												<Select.Content>
													{#if $isMac}
														<Select.Item value="Super">Command (⌘)</Select.Item>
														<Select.Item value="Alt">Option (⌥)</Select.Item>
														<Select.Item value="Control">Control (^)</Select.Item>
													{:else}
														<Select.Item value="Control">Control</Select.Item>
														<Select.Item value="Alt">Alt</Select.Item>
													{/if}
													<Select.Item value="Shift">Shift</Select.Item>
													<Select.Item value="">&nbsp;</Select.Item>
												</Select.Content>
											</Select.Root>
										</div>
										<div class="col-4">
											<Input
												type="text"
												id="shortcut-input"
												class={`flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-offset-0 focus-visible:ring-0 md:w-[150px] ${globalShortcutCode === '' ? 'border-red-500' : ''}`}
												placeholder="Key"
												on:keydown={(e) => setKeydownHandlerOnGlobalShortuctInput(e)}
												bind:value={globalShortcutCode}
											/>
										</div>
									</div>
									<div class="my-2">
										{#if globalShortcutCode === ""}
											<small class="text-danger small-explanation">Shortcut value cannot be empty</small>
											{#if globalShortcutModifiers[1].value === globalShortcutModifiers[0].value}<br/>{/if}
										{/if}
										{#if globalShortcutModifiers[1].value === globalShortcutModifiers[0].value}
											<small class="text-danger small-explanation">Both modifier keys cannot be the same</small>
										{/if}
									</div>
								</div>
								<Dialog.Footer class="flex sm:justify-between items-center gap-2">
									<Dialog.Description>Setting a new shortcut will automatically restart the app</Dialog.Description>
									<Button
												type="button"
												class="btn btn-success"
												disabled={globalShortcutCode === "" || globalShortcutModifiers[1].value === globalShortcutModifiers[0].value}
												on:click={() => setNewGlobalShortcut()}>Save</Button>
								</Dialog.Footer>
							</Dialog.Content>
						</Dialog.Root>
					will show the app from anywhere
				</div>
			</td>
			<td>
				<PopoverIcon title="Changes will take effect after the app restarts" />
			</td>
		</tr>

		<tr class="h-10">
			<td colspan="3"><Separator /></td>
		</tr>

		<tr class="h-10">
			<td colspan="3" class="text-center text-muted-foreground font-mono"><small>ADVANCED</small></td>
		</tr>

		<tr>
			<td class="text-center px-2">
				<Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={automaticBackgroundSyncEnabled} on:click={() => toggleAutomaticBackgroundSync()} />
			</td>
			<td class="py-2 skip-hover">
				Allow Automatic Background Scan
				<div class="flex items-center small-explanation gap-1">
					<div>This allows Buzee to scan your files automatically (twice an hour)</div>
				</div>
			</td>
			<td>
				<PopoverIcon title="We recommend keeping this setting enabled" />
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={detailedScanEnabled} on:click={() => toggleDetailedScan()} />
			</td>
			<td class="py-2 skip-hover">
				Scan File Text
				<div class="flex items-center small-explanation gap-1">
					<div>Keep this on so you can search inside files (PDFs are scanned last)</div>
				</div>
			</td>
			<td>
				<PopoverIcon title="Disabling this setting may improve speed but reduce quality of search results"/>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={skipParsingPDF} on:click={() => toggleSkipParsingPDF()} />
			</td>
			<td class="py-2 skip-hover">
				Skip Scanning Text from PDFs and Images
				<div class="flex items-center small-explanation gap-1">
					<div>PDFs and images take time to scan. Disable this if you don't have too many of them.</div>
				</div>
			</td>
			<td>
				<PopoverIcon title="Disabling this setting may improve the quality of search results but make the app buggy"/>
			</td>
		</tr>
		<tr>
			<td class="text-center px-2">
				<Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={manualSetupMode} on:click={() => toggleManualSetupMode()} />
			</td>
			<td class="py-2 skip-hover">
				Run in Manual Setup Mode
				<div class="flex items-center small-explanation gap-1">
					<div>In manual mode, Buzee will only sync the files and folders that you add yourself.</div>
				</div>
			</td>
			<td>
				<PopoverIcon title="Disabling this setting will make Buzee scan your entire system automatically"/>
			</td>
		</tr>
		<tr class="hover:text-red-500">
			<td class="text-center px-2">
				<!-- <Switch class="hover:data-[state=checked]:bg-violet-500" bind:checked={manualSetupMode} on:click={() => toggleManualSetupMode()} /> -->
				<Dialog.Root bind:open={clearIndexDialogOpen}>
					<Dialog.Trigger class="flex justify-center items-center w-full">
						<TriangleAlert class="h-6 w-6" />
					</Dialog.Trigger>
					<Dialog.Content>
						<Dialog.Header>
							<Dialog.Title>Clear Index</Dialog.Title>
							<Dialog.Description>Remember to run the background sync after clearing the index</Dialog.Description>
						</Dialog.Header>
						{#if $statusMessage === "Cleared!"}
							<p>Done!</p>
							<div class="flex justify-center items-center">
								<lottie-player src="/checkmark-done.json" background="transparent"  speed="1"  style="width: 200px; height: 200px;" autoplay></lottie-player>
							</div>
						{:else}
							<p class="mb-0">
								If the search results are of poor quality, clearing the index and then rebuilding it can help.<br/><br/>
								Alternatively, you can clear the index, turn off the Scan File Text setting and use Buzee only for searching file metadata.
							</p>
						{/if}
						<Dialog.Footer>
							<Dialog.Close asChild let:builder>
								<Button variant="secondary" aria-label="Close" builders={[builder]}>Close</Button>
							</Dialog.Close>
							<Button on:click={() => clearIndex()}>Yes, clear the index</Button>
						</Dialog.Footer>
					</Dialog.Content>
				</Dialog.Root>
			</td>
			<td class="py-2 skip-hover" role="button" on:click={() => {clearIndexDialogOpen = true;}}>
				Clear the Index
				<div class="flex items-center small-explanation gap-1">
					<div>If the search results are of poor quality, clearing the index can help.</div>
				</div>
			</td>
			<td>
				
			</td>
		</tr>

		<tr class="h-10">
			<td colspan="3"><Separator /></td>
		</tr>
	</table>
	<div class="w-4/5 md:w-3/5 flex justify-between settings-links">
		<div class="relative flex-grow max-w-full flex-1 px-4 text-start mobile-text-center my-1">
			<Button variant="link" class="gap-2 text-xs !px-2" on:click={() => resetDefault()}>
				<span class="font-normal text-xs">Reset Default</span>
				<PopoverIcon title="Reset all settings to default and restart the app. This will NOT clear the database." />
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
		}
	}

	tr {
		cursor: default;
		// &:not(.skip-hover):hover {
		// 	cursor: default;
		// 	color: var(--purple);
		// }
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
