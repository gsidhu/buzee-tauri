<script lang="ts">
  export let item: DropdownOrButtonItem;
  let handleClick = item.handleClick ? item.handleClick : () => {};
	let icon = item.icon ? item.icon : '';
	let id = item.id ? item.id : '';
	let label = item.label;
	let marginClass = item.marginClass ? item.marginClass : 'm-1';
	let paddingClass = item.paddingClass ? item.paddingClass : 'px-1 py-1';
	let showIcon = item.showIcon ? item.showIcon : false;
	let showText = item.showText ? item.showText : true;
  let title = item.title ? item.title : '';
	let type = item.type ? item.type : 'dropdown-item';
	let isDisabled = item.isDisabled ? item.isDisabled : false;
</script>

<li id={id} class={`${marginClass}`}>
  <button 
    class={`btn ${paddingClass} ${type}`} 
    title={`${title !== '' ? title : label}`}
    on:click={() => handleClick()}
    disabled={isDisabled}
  >
    {#if showIcon}
      <i class={`bi ${icon} ${showText ? 'margin-right-75' : ''}`} />
    {/if}
    {#if showText}
      {label}
    {/if}
  </button>
</li>

<style lang="scss">
  button {
    font-size: 0.8rem;
  }
  .dropdown-item {
    border-radius: 4px;

    &:hover {
      background-color: var(--purple);
      color: white;
    }
  }
</style>