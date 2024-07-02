<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from 'svelte';
	import { documentsShown, base64Images, shiftKeyPressed, compactViewMode, selectedResult, showResultTextPreview, noMoreResults } from '$lib/stores';
	import FileTypeIcon from '$lib/components/ui/FileTypeIcon.svelte';
	import { stringToHash } from '$lib/utils/miscUtils';
	import { clickRow } from '$lib/utils/fileUtils';
	import { trackEvent } from '@aptabase/web';
	import { goto } from "$app/navigation";
	import { startDragging, openFile } from "$lib/utils/searchItemUtils";
	import * as ContextMenu from "$lib/components/ui/context-menu";

	// $: if ($documentsShown.length < 50) {
	// 	$noMoreResults = true;
	// }

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
	<div class="flex flex-col px-4 py-2 mx-auto items-center justify-center min-vh-80">
		<img id="buzee-logo-img" class="w-25 my-2" src="/Buzee Logo.png" alt="No Results" />
		<h3>No Results</h3>
		<div class="flex flex-col text-light-emphasis text-center small gap-2">
			<span>Try modifying your query? You can be more specific like –</span>
			<span><code>last year "annual report" -pdf</code></span>
		</div>
		<button type="button" class="my-2 btn py-1 px-2 leading-tight text-xs purple border-hover-purple border-2 border-gray-100 rounded" on:click={() => goto('/magic/tips')}>View all tips and shortcuts</button>
	</div>
{:else}
<div id="parent-grid" class="flex flex-col">
  <div class={`file-grid p-2 ${$compactViewMode ? 'gap-2' : 'gap-4'}`}>
    {#each $documentsShown as result, i}
			<ContextMenu.Root>
				<ContextMenu.Trigger>
				<button
					id={stringToHash($documentsShown[Number(i)].path)}
					style="all: unset;"
					class={`icon-item w-full h-full p-1 grid items-center justify-between table-row result-${Number(i)} ${$compactViewMode ? 'compact-view' : ''}`}
					tabindex="0"
					on:focus={(e) => clickRow(e, $shiftKeyPressed)}
					on:click={(e) => clickRow(e, $shiftKeyPressed)}
					on:dblclick={() => openFile($documentsShown[Number(i)].path)}
					draggable="true"
					on:dragstart={(event) => startDragging($documentsShown[Number(i)].path)}
					title={result.name}
				>
					<div class="flex justify-center">
						{#if ['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp'].includes(result.file_type)}
							<img src={"data:image/png;base64, " + $base64Images[result.path]} alt={result.name} class={`img-thumbnail ${$compactViewMode ? 'compact-view' : ''}`} />
						{:else}
							<FileTypeIcon filetype={result.file_type} extraClasses={`${$compactViewMode ? 'text-lg' : 'text-2xl'}`}/>
						{/if}
					</div>
					<div class="filename text-center p-1 w-full">
						{result.name}
					</div>
				</button>
			</ContextMenu.Trigger>
			<ContextMenu.Content>
				{#if result.file_type !== 'folder' && result.last_parsed !== 0}
					<ContextMenu.Item on:click={() => {$showResultTextPreview = true; $selectedResult = result;}}>
						Show Preview
					</ContextMenu.Item>
				{/if}
				<ContextMenu.Item>
					Open {result.file_type === 'folder' ? 'Folder' : 'File'}
				</ContextMenu.Item>
				<ContextMenu.Sub>
					<ContextMenu.SubTrigger>Ignore</ContextMenu.SubTrigger>
					<ContextMenu.SubContent class="w-48">
						<ContextMenu.Item>Ignore this {result.file_type === 'folder' ? 'folder' : 'file'}</ContextMenu.Item>
						<ContextMenu.Item>Ignore parent folder</ContextMenu.Item>
						{#if result.file_type !== 'folder'}
							<ContextMenu.Item>Ignore this file's text</ContextMenu.Item>
						{/if}
					</ContextMenu.SubContent>
				</ContextMenu.Sub>
			</ContextMenu.Content>
		</ContextMenu.Root>
    {/each}
  </div>
</div>
{/if}

<style lang="scss">
  #parent-grid {
    overflow-x: hidden; /* Hide horizontal scrollbar */
    overflow-y: auto; /* Enable vertical scrolling */
    max-height: 60svh;
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