<script lang="ts">
  import DropdownOption from "./dropdownOption.svelte";
  export let dropdownItems: DropdownItemsArray;
  
</script>

<div class="dropdown">
  <button
    type="button"
    class="btn btn-sm search-options-menu-btn px-1"
    data-bs-toggle="dropdown"
    aria-expanded="false"
  >
    <i class="bi bi-three-dots" />
  </button>
  <ul class="dropdown-menu p-0">
    {#each dropdownItems as item}
      <DropdownOption item={item} />
    {/each}
  </ul>
</div>

<style lang="scss">
  .dropdown-menu {
    font-size: 0.8rem;
    background-color: var(--search-bg);
  }
  .search-options-menu-btn {
    all: unset;
    cursor: pointer;
  }
</style>