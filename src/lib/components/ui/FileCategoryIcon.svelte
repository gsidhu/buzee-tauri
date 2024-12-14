<script lang="ts">
  import {FileMusic, BookOpenText, FileText, Folder, Image, Film, SearchCheck, Laptop, Cloud, Compass, Bookmark } from "lucide-svelte";

  export let category: string;
  export let className: string = "mr-2 h-4 w-4";
</script>

{#if category === "audio"}
  <FileMusic class={className} />
{:else if category === "book"}
  <BookOpenText class={className} />
{:else if category === "document"}
  <FileText class={className} />
{:else if category === "folder"}
  <Folder class={className} />
{:else if category === "image"}
  <Image class={className} />
{:else if category === "video"}
  <Film class={className} />
{:else if category === "my computer"}
  <Laptop class={className} />
{:else if category === "cloud"}
  <Cloud class={className} />
{:else if category === "browser history"}
  <Compass class={className} />
{:else if category === "bookmarks"}
  <Bookmark class={className} />
{:else if category === "any"}
  <SearchCheck class={className} />
{/if}