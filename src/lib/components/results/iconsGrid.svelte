<script lang="ts">
	import { invoke, transformCallback } from "@tauri-apps/api/core";
	import moment from 'moment';
	import { onMount } from 'svelte';
	import { documentsShown, base64Images, shiftKeyPressed, compactViewMode, selectedResult, resultsPageShown, searchInProgress, filetypeShown, allowedExtensions, searchQuery, resultsPerPage } from '$lib/stores';
	import { searchDocuments } from '$lib/utils/dbUtils';
	import { setExtensionCategory } from '$lib/utils/miscUtils';
	import FileTypeIcon from '$lib/components/ui/FileTypeIcon.svelte';
	import { stringToHash, readableFileSize, resetColumnSize } from '$lib/utils/miscUtils';
	import { clickRow } from '$lib/utils/fileUtils';
	import { trackEvent } from '@aptabase/web';
	import ConfettiButton from '../ui/confettiButton.svelte';
	import { goto } from "$app/navigation";

	let noMoreResults = false;

	$: if ($documentsShown.length < 50) {
		noMoreResults = true;
	}

	async function loadMoreResults() {
		// Same function as triggerSearch, but with a different page number and appending results
		console.log("Loading more results...");
		$resultsPageShown += 1; // increment the page number on each new search
		$searchInProgress = true;
		trackEvent('loadMoreResults', {
			filetypeShown: $filetypeShown,
			resultsPageShown: $resultsPageShown
		});
		let filetypeToGet = $filetypeShown;
		if (filetypeToGet !== 'any') {
			filetypeToGet = setExtensionCategory($filetypeShown, $allowedExtensions);
		}
		let results = await searchDocuments(
			$searchQuery,
			$resultsPageShown,
			$resultsPerPage,
			filetypeToGet
		);
		if (results.length === 0) {
			noMoreResults = true;
		} else {
			$documentsShown = [...$documentsShown, ...results];
		}
		$searchInProgress = false;
	}
  
  function startDragging(filepath: string) {
		const image64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAA7AAAAOwBeShxvQAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAADySURBVFiF7dcxSgNRFIXhT4Wx1NrCFCK4iEiCWxBcgjsQscteLMQlKCksLCLYWbkE3cEEJBaTgL5MzGReAhb3h9cc3tz7w0wxh3k6GKLEpOV5R7dmdiOGuEHR8vkBHvGBfpsBZcbymcBgunypxHZNVmCcITDjCee4x9kqAuvkeSpx95dEyiRz6SVuk6yPT5yml7cWCNTlTdnHK0Z4+5F3cYyTTQvAHi5wlOTXTWbnvoKVZm/6I1xKCIRACIRACIRACIRACPxLgbG8araIXVXt+8VOzcUeDvCCrzUtL3Cl+iVPS8scHVW7zann6SnxgMN02Ter0UNOfhP2XAAAAABJRU5ErkJggg==";
    startDrag({ item: [filepath], icon: image64 })
  }

  async function startDrag(
    options: Options,
    onEvent?: (result: CallbackPayload) => void
  ): Promise<void> {
    await invoke("start_drag", {
      item: options.item,
      image: options.icon,
      onEventFn: onEvent ? transformCallback(onEvent) : null,
    });
  }

	function openFile(url: string) {
		trackEvent('click:openFile');
		invoke('open_file_or_folder', { filePath: url });
	}
	
	function formatUpdatedTime(unixTime: number): string {
		if (unixTime === 0) {
			return 'Never';
		}
		let unixToJs = new Date(unixTime*1000);
		const updatedMoment = moment(unixToJs);
		const today = moment();
		const yesterday = moment().subtract(1, 'days');

		if (updatedMoment.isSame(today, 'day')) {
			// If the update was today, return the time
			return updatedMoment.format('h:mm A');
		} else if (updatedMoment.isSame(yesterday, 'day')) {
			// If the update was yesterday, return 'Yesterday'
			return 'Yesterday';
		} else {
			// Otherwise, return the date
			return updatedMoment.format('YYYY-MM-DD');
		}
	}

	function formatPath(url: string): string {
		const parts = url.split('/'); // Split the url into components
		const length = parts.length;

		if (length > 3) {
			// Take the two directories just before the file name and prepend '...'
			return '.../' + parts.slice(length - 3, length - 1).join('/') + '/';
		} else {
			// If the url is already short, return it as is
			return url;
		}
	}

	async function showContextMenu(
		e: MouseEvent & { currentTarget: EventTarget & HTMLDivElement }
    | (FocusEvent & { currentTarget: EventTarget & HTMLButtonElement }),
		result: DocumentSearchResult
	) {
		trackEvent('right_click:resultContextMenu');
		clickRow(e, $shiftKeyPressed);
		// window.menuAPI?.showResultsContextMenu(result);
		$selectedResult = result;
		await invoke("open_context_menu", {option:"searchresult"});
	}

  async function getBase64Image(filePath: string) {
    return await invoke('get_image_base64', { filePath });
  }

  onMount(() => {
		// select the first result when loading new search results
		$selectedResult = $documentsShown[0];
		let firstResult = document.querySelector('.result-0') as HTMLElement | null;
		if (firstResult) {
			firstResult.focus();
		}

    $documentsShown.forEach(async (result) => {
      if (['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp'].includes(result.file_type) && 
          !$base64Images[result.path]) {
        getBase64Image(result.path).then((res) => {
          // @ts-ignore
          $base64Images[result.path] = res;
        })
      }
    });
  });
</script>

{#if $documentsShown.length <= 0}
	<div class="d-flex flex-column px-4 py-2 mx-auto align-items-center justify-content-center min-vh-80">
		<img id="buzee-logo-img" class="w-25 my-2" src="/Buzee Logo.png" alt="No Results" />
		<h3>No Results</h3>
		<div class="d-flex flex-column text-light-emphasis text-center small gap-2">
			<span>Try modifying your query? You can be more specific like –</span>
			<span><code>last year "annual report" -pdf</code></span>
		</div>
		<button type="button" class="my-2 btn btn-sm purple border-hover-purple border-2 border-light rounded" on:click={() => goto('/magic/tips')}>View all tips and shortcuts</button>
	</div>
{:else}
<div id="parent-grid" class="d-flex flex-column">
  <div class="file-grid gap-2 p-2">
    {#each $documentsShown as result, i}
      <button
        id={stringToHash($documentsShown[Number(i)].path)}
        style="all: unset;"
        class={`icon-item p-1 d-flex flex-column align-items-center justify-content-between table-row result-${Number(i)} ${$compactViewMode ? 'compact-view' : ''}`}
        tabindex="0"
        on:focus={(e) => clickRow(e, $shiftKeyPressed)}
        on:click={(e) => clickRow(e, $shiftKeyPressed)}
        on:contextmenu={(e) => showContextMenu(e, $documentsShown[Number(i)])}
        on:dblclick={() => openFile($documentsShown[Number(i)].path)}
        draggable="true"
        on:dragstart={(event) => startDragging($documentsShown[Number(i)].path)}
        title={result.name}
      >
        <div class="w-100 h-100 d-flex align-items-center justify-content-center">
          {#if ['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp'].includes(result.file_type)}
            <img src={"data:image/png;base64, " + $base64Images[result.path]} alt={result.name} class={`img-thumbnail ${$compactViewMode ? 'compact-view' : ''}`} />
          {:else}
            <FileTypeIcon filetype={result.file_type} extraClasses={`${$compactViewMode ? 'fs-4' : 'fs-1'}`}/>
          {/if}
        </div>
        <div class="filename text-center p-1 w-100">
          {result.name}
        </div>
      </button>
    {/each}
  </div>
  {#if !noMoreResults}
    <div id="load-more-btn" class="py-2 d-flex justify-content-center align-items-center" draggable="false">
      <ConfettiButton 
        label="Load more"
        type="confetti-button btn-sm"
        showText={!$searchInProgress}
        showSpinner={$searchInProgress}
        handleClick={() => loadMoreResults()} />
    </div>
  {/if}
</div>
{/if}

<style lang="scss">
  #parent-grid {
    overflow-x: hidden; /* Hide horizontal scrollbar */
    overflow-y: auto; /* Enable vertical scrolling */
    max-height: calc(100vh - 80px); /* set maximum height so rows don't hide outside the viewport; 110px is roughly the height of the topbar + statusbar + loadMore button */
  }
  .img-thumbnail {
    max-height: 72px;
    max-width: 96px;
  }
  .img-thumbnail.compact-view {
    max-height: 48px;
    max-width: 64px;
  }
  .file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  }
  .filename {
    font-size: 0.75rem;
    width: 100px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .icon-item {
    width: 100px !important;
    height: 100px !important;
  }
  .icon-item.compact-view {
    width: 70px !important;
    height: 70px !important;
  }
  .min-vh-80 {
		min-height: 80vh !important;
	}
	#buzee-logo-img {
		max-width: 200px;
	}
  // selected row
	.selected {
		background-color: var(--purple) !important;
		color: white;
		.pinned,
		.pin,
    .filename {
			color: white;
		}
		&.grayscale {
			filter: grayscale(.7);
			background-color: transparent !important;
		}
	}
	// pinned rows
	.pinned {
		color: var(--hot-pink);
	}
	.pin {
		color: var(--bs-body-color);
	}
	.pin:hover {
		color: var(--hot-pink);
	}
</style>