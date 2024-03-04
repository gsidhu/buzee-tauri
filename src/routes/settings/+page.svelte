<script lang="ts">
	import { fade } from 'svelte/transition';
	import TopBar from '../../layout/TopBar.svelte';
	import PopoverIcon from '$lib/components/ui/popoverIcon.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { sendEvent } from '../../utils/firebase';

	let showInDock: boolean;
	let launchAtStartup: boolean;
	let globalShortcutsEnabled: boolean;
	let automaticBackgroundSyncEnabled: boolean;
	let isMac: boolean = true;
	let isWin: boolean;
	let userPreferences: UserPreferences;

	async function getUserPreferences() {
		return await window.settingsAPI?.getUserPreferences();
	}

	function toggleDockIcon() {
		showInDock = !showInDock;
		sendEvent('click:toggleDockIcon', { showInDock });
		window.settingsAPI?.toggleDockIcon();
	}

	function toggleLaunchAtStartup() {
		launchAtStartup = !launchAtStartup;
		sendEvent('click:toggleLaunchAtStartup', { launchAtStartup });
		window.settingsAPI?.toggleLaunchAtStartup();
	}

	function toggleGlobalShortcuts() {
		globalShortcutsEnabled = !globalShortcutsEnabled;
		sendEvent('click:toggleGlobalShortcuts', { globalShortcutsEnabled });
		window.settingsAPI?.toggleGlobalShortcuts();
	}

	function toggleAutomaticBackgroundSync() {
		automaticBackgroundSyncEnabled = !automaticBackgroundSyncEnabled;
		sendEvent('click:toggleAutomaticBackgroundSync', { automaticBackgroundSyncEnabled });
		window.settingsAPI?.toggleAutomaticBackgroundSync();
	}

	function resetDefault() {
    sendEvent('click:resetDefault');
		window.settingsAPI?.resetDefault();
	}

	function uninstallApp() {
    sendEvent("click:uninstallApp");
		goto('/uninstall');
	}

	function addDocsToDB() {
		sendEvent('click:addDocsToDB');
		window.dbAPI?.addDocsToDB();
	}

	onMount(() => {
		isMac = window.constants.isMac();
		isWin = window.constants.isWin();
		getUserPreferences().then((res) => {
			userPreferences = res;
			console.log(userPreferences);
			showInDock = userPreferences.showInDock; 
			launchAtStartup = userPreferences.launchAtStartup;
			globalShortcutsEnabled = userPreferences.globalShortcutsEnabled;
			automaticBackgroundSyncEnabled = userPreferences.automaticBackgroundSync;
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
							title="By default, Buzee scans files in your Documents, Downloads and Desktop folders"
						/>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2"
					><input type="checkbox" bind:checked={showInDock} on:click={() => toggleDockIcon()} /></td
				>
				<td class="py-2" on:click={() => toggleDockIcon()}>
					{#if isMac}
						Show in Dock
						<div class="d-flex align-items-center small-explanation gap-1">
							Whether to show the app icon in the dock
							<PopoverIcon
								title="The app icon always appears on restarts, then follows user settings"
							/>
						</div>
					{:else if isWin}
						Show in Taskbar
						<div class="d-flex align-items-center small-explanation gap-1">
							Whether to show the app icon in the taskbar
						</div>
					{:else}
						Show in Taskbar
					{/if}
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
						Whether to launch the app when you restart your computer
					</div>
				</td>
			</tr>
			{#if isMac || isWin}
				<tr>
					<td class="text-center px-2"
						><input
							type="checkbox"
							bind:checked={globalShortcutsEnabled}
							on:click={() => toggleGlobalShortcuts()}
						/>
					</td>
					<td class="py-2" on:click={() => toggleGlobalShortcuts()}>
						Allow Global Shortcut
						<div class="d-flex align-items-center small-explanation gap-1">
							{#if isMac}
								<div>Pressing <code>Option+Space</code> will show the app from anywhere</div>
								<PopoverIcon title="Changes will take effect after the app restarts" />
							{:else if isWin}
								<div>Pressing <code>Alt+Space</code> will show the app from anywhere</div>
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
					Allow Automatic Background Sync
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Buzee will automatically sync your data in the background periodically</div>
						<PopoverIcon title="We recommend keeping this setting enabled" />
					</div>
				</td>
			</tr>
		</table>
		<div class="d-flex w-90 justify-content-between small-explanation">
			<div>
				<button class="btn btn-sm link-danger px-0" on:click={() => resetDefault()}
					>Reset Default</button
				>
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
