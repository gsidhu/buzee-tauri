<script lang="ts">
  import { onMount, onDestroy } from "svelte";
	import FileTypeIcon from "./FileTypeIcon.svelte";

  export let icon = 'bi-question-circle';
  export let id = '';
  export let label = '';
  export let marginClass = 'm-0';
  export let paddingClass = 'p-0';
  export let showIcon = true;
  export let showText = false;
  export let title = '';
  export let isBtn = true;
  export let filetypeicon = false;

  onMount(() => {
    const tooltipTriggerList = document.querySelectorAll('[data-bs-toggle="tooltip"]')
    // ignoring ts-check here because bootstrap is not a module, it is added via script tag
    // @ts-ignore
    const tooltipList = [...tooltipTriggerList].map(tooltipTriggerEl => new bootstrap.Tooltip(tooltipTriggerEl, {boundary: document.body}))
  });
  onDestroy(() => {
    // remove any stray tooltips from the DOM
    const tooltipList = document.querySelectorAll('.custom-tooltip');
    tooltipList.forEach((tooltip) => { tooltip.remove() });
  });
</script>

{#if isBtn}
<button id={id} type="button" class={`btn ${paddingClass} ${marginClass}`} 
    data-bs-toggle="tooltip" data-bs-placement="top"
    data-bs-custom-class="custom-tooltip"
    data-bs-title={`${title}`}
  >
  {#if showIcon}
    {#if filetypeicon}
      <FileTypeIcon filetype={icon} color={false}/>
    {:else}
      <i class={`bi ${icon} ${showText ? 'margin-right-75' : ''}`} />
    {/if}
  {/if}
  {#if showText}
    {label}
  {/if}
</button>
{:else}
<span id={id} class={`${paddingClass} ${marginClass}`} 
    data-bs-toggle="tooltip" data-bs-placement="top"
    data-bs-custom-class="custom-tooltip"
    data-bs-title={`${title}`}
  >
  {#if showIcon}
    <i class={`bi ${icon} ${showText ? 'margin-right-75' : ''}`} />
  {/if}
  {#if showText}
    {label}
  {/if}
</span>
{/if}

<style lang="scss">
  button {
    font-size: inherit !important;
    border: none !important;
    color: inherit !important;

    &:hover {
      color: var(--bs-dark) !important;
    }
  }
  // styles for the path column in results; same as in svelteTable.svelte
  span {
    display: block;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>