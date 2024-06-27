<script lang="ts">
	import FileTypeIcon from "./FileTypeIcon.svelte";
  import * as Tooltip from "$lib/components/ui/tooltip";

  export let icon = 'bi-question-circle';
  export let label = '';
  export let showIcon = true;
  export let showText = false;
  export let title = '';
  export let filetypeicon = false;
</script>

<Tooltip.Root>
  <Tooltip.Trigger>
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
  </Tooltip.Trigger>
  <Tooltip.Content>{title}</Tooltip.Content>
</Tooltip.Root>

<style lang="scss">
  i:hover {
    color: var(--purple);
  }
</style>