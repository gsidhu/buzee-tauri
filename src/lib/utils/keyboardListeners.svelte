<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from 'svelte';
	import { clickRow, selectOneRow, selectAllRows } from './fileUtils';
	import { isMac, documentsShown, shiftKeyPressed, metaKeyPressed } from '$lib/stores';
	import { trackEvent } from '@aptabase/web';
	import { goto } from '$app/navigation';

	const allowedKeys = [
		'Space',
		'Enter',
		'KeyO',
		'KeyA',
		'KeyF',
		'KeyK',
		'KeyS',
		'ShiftLeft',
		'ShiftRight',
		'Tab',
		'KeyP',
		'MetaLeft',
		'MetaRight',
		'ArrowDown',
		'ArrowUp',
		'Escape',
		'CommandOrControl+A'
	];

	const eventPrefix = 'keyboardListener:';

	function keydownListener(e: KeyboardEvent) {
		if (e.code === 'MetaLeft' || e.code === 'MetaRight' || e.code === 'ControlLeft' || e.code === 'ControlRight') {
			console.log('meta key down');
			$metaKeyPressed = true;
		}
		if (e.code === 'ShiftLeft' || e.code === 'ShiftRight') {
			console.log('shift key down');
			$shiftKeyPressed = true;
		}
		if (document.activeElement instanceof HTMLInputElement) return;
		if (allowedKeys.indexOf(e.code) < 0) return;

		// Easter Egg: Go straight to Scratch Pad
		if ($metaKeyPressed && $shiftKeyPressed && e.code === 'KeyS') {
			e.preventDefault();
			trackEvent(eventPrefix + 'goToScratchPad');
			goto('/magic/scratchpad');
			return;
		}

		if ($metaKeyPressed && e.code === 'KeyA') {
			e.preventDefault();
			trackEvent(eventPrefix + 'selectAllRow');
			selectAllRows(false);
			return;
		}

		if ($metaKeyPressed && (e.code === 'KeyF' || e.code === 'KeyK')) {
			e.preventDefault();
			if ($shiftKeyPressed) {
				console.log('Cmd + Shift + F');
				trackEvent(eventPrefix + 'toggleAppMode');
				window.electronAPI?.toggleAppMode();
			} else {
				console.log('Cmd + F');
				trackEvent(eventPrefix + 'focusSearchBar');
				// if page is not /search, go to that page
				if (window.location.pathname !== '/search') {
					goto('/search?highlight-search-bar=true');
				}
				(document.querySelector('#search-input') as HTMLElement)?.click();
			}
			return;
		}

		if (e.code === 'Escape') {
			e.preventDefault();
			trackEvent(eventPrefix + 'deselectAllRows');
			selectAllRows(true);
			document.body.focus();
			return;
		}

		const selectedElement = document.activeElement as HTMLElement;
		let thisResultIndex: string | undefined = '-1';

		// If a result is selected
		if (selectedElement?.classList.contains('selected')) {
			thisResultIndex = Array.from(selectedElement?.classList)
				.find((className) => className.startsWith('result-'))
				?.split('-')[1];
			let result = $documentsShown[Number(thisResultIndex)];

			console.log("thisResultIndex:", thisResultIndex);
			console.log("meta key pressed:", $metaKeyPressed);

			if (e.code === 'Space') {
				e.preventDefault();
				trackEvent(eventPrefix + 'openQuickLook');
				// window.electronAPI?.openQuickLook(result.path);
				invoke("open_quicklook", { filePath: result.path })
			} else if (e.code === 'Enter') {
				e.preventDefault();
				trackEvent(eventPrefix + 'openFile');
				// window.electronAPI?.openFile(result.path);
				invoke("open_file_or_folder", { filePath: result.path })
			} else if (e.code === 'ArrowDown' && $metaKeyPressed && $isMac) {
				e.preventDefault();
				// window.electronAPI?.openFile(result.path);
				invoke("open_file_or_folder", { filePath: result.path })
			} else if (e.code === 'KeyO') {
				e.preventDefault();
				// window.electronAPI?.openFileFolder(result.path);
				invoke("open_folder_containing_file", { filePath: result.path })
			} else if (e.code === 'Tab' && $shiftKeyPressed) {
				$shiftKeyPressed = false;
			} else if (e.code === 'KeyP') {
				e.preventDefault();
				// togglePinState();
			} else if (e.code === 'ArrowUp') {
				e.preventDefault();
				if (document.getElementsByClassName('selected').length > 2) {
					trackEvent(eventPrefix + 'deselectAllRows');
					selectAllRows(true);
				}
				if ($shiftKeyPressed) {
					// Have to find which is the "highest" result selected
					// const lastSelected = document.getElementsByClassName("selected")[0];
					// const lastSelectedIndex = Array.from(lastSelected?.classList).find((className) => className.startsWith("result-"))?.split("-")[1];
					// let prevIndex = (Number(lastSelectedIndex) ?? 0) - 1;
					// let prevElement = document.getElementsByClassName("result-" + prevIndex.toString())[0];
					// if (prevElement) {
					//   selectOneRow(prevElement as HTMLDivElement);
					// }
				} else {
					let prevElement = selectedElement.parentElement?.previousElementSibling?.children[0] as HTMLDivElement;
					clickRow(
						{ currentTarget: prevElement } as MouseEvent & {
							currentTarget: EventTarget & HTMLDivElement;
						},
						$shiftKeyPressed
					);
				}
				return;
			} else if (e.code === 'ArrowDown') {
				e.preventDefault();
				if (document.getElementsByClassName('selected').length > 2) {
					trackEvent(eventPrefix + 'deselectAllRows');
					selectAllRows(true);
				}
				if ($shiftKeyPressed) {
					// Have to find which is the "lowest" result selected
					// const lastSelected = document.getElementsByClassName("selected")[document.getElementsByClassName("selected").length - 1];
					// const lastSelectedIndex = Array.from(lastSelected?.classList).find((className) => className.startsWith("result-"))?.split("-")[1];
					// let nextIndex = (Number(lastSelectedIndex) ?? 0) + 1;
					// let nextElement = document.getElementsByClassName("result-" + nextIndex.toString())[0];
					// if (nextElement) {
					//   selectOneRow(nextElement as HTMLDivElement);
					// }
				} else {
					let nextElement = selectedElement.parentElement?.nextElementSibling?.children[0] as HTMLDivElement;
					clickRow(
						{ currentTarget: nextElement } as MouseEvent & {
							currentTarget: EventTarget & HTMLDivElement;
						},
						$shiftKeyPressed
					);
				}
				return;
			}
		}
	}

	function keyupListener(e: KeyboardEvent) {
		if (e.code === 'ShiftLeft' || e.code === 'ShiftRight') {
			console.log('shift key up');
			$shiftKeyPressed = false;
		}
		if (e.code === 'MetaLeft' || e.code === 'MetaRight' || e.code === 'ControlLeft' || e.code === 'ControlRight') {
			console.log('meta key up');
			$metaKeyPressed = false;
		}
		// HACK to prevent meta and shift key values from getting stuck
		if (['MetaLeft','MetaRight','ControlLeft','ControlRight','ShiftLeft','ShiftRight'].indexOf(e.code) < 0) {
			$metaKeyPressed = false;
			$shiftKeyPressed = false;
		}
	}

	onMount(() => {
		document.addEventListener('keyup', keyupListener);
		document.addEventListener('keydown', keydownListener);
	});
</script>
