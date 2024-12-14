<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  
  import { isMac } from '$lib/stores';

  onMount(() => {
  })

</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Tips & Shortcuts</h3>
  <p class="text-sm text-muted-foreground">Learn how to get the most out of Buzee</p>
</div>
<div class="p-4 w-full items-center justify-center rounded-lg border border-dashed shadow-sm text-sm">
  <h5>Keyboard Shortcuts</h5>
  <ul class="list-disc">
    <li>Press <code>{$isMac ? "⎇" : "Alt"} + Space</code> anywhere to show/hide the app</li>
    <li>In the app, press <code>{$isMac ? "⌘" : "Ctrl"} + F or K</code> to go the search bar</li>
    <li>Press <code>Enter</code> on a selected row to open the document</li>
    <li>Press <code>O</code> on a selected row to open the folder containing the document</li>
    {#if $isMac}
      <li>Press <code>Space</code> on a selected row to quickly preview the document</li>
    {/if}
    <li>Use the <code>↑ ↓</code> arrow keys to move between search results</li>
  </ul>

  <h5>Keeping Buzee Updated</h5>
  <p>Buzee automatically scans data from your computer. You can always click the <code><i class="bi bi-arrow-repeat"></i></code> button at the top to run it manually.</p>
  <p>By default, Buzee scans files in your Documents, Downloads and Desktop folders. You can add more documents to search in Buzee using the <code>Add Documents</code> option in the <a href="/settings" class="purple">Settings</a>.</p>
  <p>Ignore specific files or complete folders by adding them to the <code>Ignore List</code> in the Settings. You can right-click on a search result to ignore it as well.</p>

  <h5>Include/Exclude from Search</h5>
  <ul class="list-disc">
    <li>Use the Filetype filter or simply type it in the search (like <code>invoice pdf</code>)</li>
    <li>Put quotes around keyword(s) to search for the exact phrase (like <code>"annual report"</code>)</li>
    <li>Put a hyphen in front of the keyword to exclude it from search (like <code>"annual report" -2022 -pdf</code>)</li>
  </ul>

  <h5>Search using Time</h5>
  <p>Use the Date Range filter or simply mention the date/time period that you are looking for in your search. For example:</p>
  <ul class="list-disc">
    <li><code><strong>last month</strong> pdf invoice</code></li>
    <li><code>annual report ppt <strong>this year</strong></code></li>
    <li><code><strong>q2 2023 to q3 2023</strong> retail report xlsx</code></li>
    <li><code>prelim findings <strong>from 2017 to 2022</strong></code></li>
    <li><code>cv docx <strong>from march 2 2020 to aug 15 2020</strong> -pdf</code></li>
    <li><code>scope study <strong>14/02/2015 to 10/08/2015</strong></code></li>
  </ul>
  <p>If you want to use a phrase as a keyword for search and not time, simply wrap it in quotes like this: <code>invoice "March 2022"</code></p>
</div>