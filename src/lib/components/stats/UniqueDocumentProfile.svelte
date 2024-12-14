<script lang="ts">
	import FileTypeIcon from "../ui/FileTypeIcon.svelte";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import PopoverIcon from '../ui/popoverIcon.svelte';
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
	import Separator from "../ui/separator/separator.svelte";

  let dbStats: DBStat[] = [];
  let totalItems = 0;
  let totalDocs = 0;
  let filesParsed = 0;
  let docStats: DBStat[] = [];
  let statPercentage: DBStat[] = [];
  const officePDFTypes = ['doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'pdf'];
	const allDocTypes = [...officePDFTypes, 'pages', 'numbers', 'key', 'txt', 'md', 'csv'];

  async function updateDBStats() {
    dbStats = await invoke('get_db_stats');
    // remove duplicates from dbStats
    dbStats = dbStats.filter((stat, index, self) =>
      index === self.findIndex((t) => (
        t.file_type === stat.file_type
      ))
    );
    
    totalItems = dbStats.reduce((acc, curr) => acc + curr.count, 0);

    dbStats.map((stat) => {
      if (allDocTypes.indexOf(stat.file_type) > -1) {
        statPercentage.push({
          file_type: stat.file_type,
          count: Math.round((100 * stat.count) / totalItems)
        });
      }
    });

    docStats = dbStats.filter((stat) => allDocTypes.indexOf(stat.file_type) > -1);
    totalDocs = docStats.reduce((acc, curr) => acc + curr.count, 0);
    
    let sumStatPercentages = statPercentage.reduce((acc, curr) => acc + curr.count, 0);
    
    statPercentage.push({
      file_type: 'other',
      count: 100 - sumStatPercentages
    });
    docStats.push({
      file_type: 'other',
      count: totalItems - totalDocs
    });

    filesParsed = await invoke("get_count_of_files_parsed");
  }

  // function saveUniqueDocumentProfile() {
  //   html2canvas(document.getElementById("unique-profile") as HTMLElement).then(canvas => {
  //     saveAs(canvas.toDataURL(), 'unique-document-profile.png');
  //   });
  // }
  // function saveAs(uri:string, filename:string) {
  //   var link = document.createElement('a');
  //   if (typeof link.download === 'string') {
  //       link.href = uri;
  //       link.download = filename;
  //       //Firefox requires the link to be in the body
  //       document.body.appendChild(link);
  //       //simulate click
  //       link.click();
  //       //remove the link when done
  //       document.body.removeChild(link);
  //   } else {
  //       window.open(uri);
  //   }
  // }

  onMount(async () => {
    await updateDBStats();
  })
</script>

<h6 class="text-center cursor-default">Your Unique Document Profile</h6>

<div class="flex overflow-hidden w-90 mx-auto rounded my-8">
  {#each docStats as stat, index}
    <div style={`width: ${statPercentage[index].count}%`} class="bar-segment">
      <Tooltip.Root>
        <Tooltip.Trigger 
          id={`${stat.file_type}-segment`}
          class="text-white text-center w-full"
          style={stat.file_type === "other"
            ? `background-color: #E15554;`
            : `background-color: var(--${stat.file_type}-icon);`}
        >
          <FileTypeIcon filetype={stat.file_type === "other" ? 'other-file-folder' : stat.file_type} color={false}/>
        </Tooltip.Trigger>
        <Tooltip.Content>{`${stat.count} ${stat.file_type} files (${statPercentage[index].count}%)`}</Tooltip.Content>
      </Tooltip.Root>
    </div>
  {/each}
</div>

<div class="w-4/5 pr-4 pl-4 mx-auto my-4 cursor-default">
  <div class="columns-1 sm:columns-2">
    {#each docStats as stat}
      {#if stat.count > 0}
        <div class="columns-2">
          <div class="">
            <FileTypeIcon filetype={stat.file_type === "other" ? 'folder' : stat.file_type} onlyIcon={true}/>
            <small>{stat.file_type}</small>
          </div>
          <div class="text-end"><small>{stat.count}</small></div>
        </div>
      {/if}
    {/each}
  </div>
  <Separator class="my-2" />
  <div class="columns-1 justify-center">
    <div class="columns-2">
      <div class="">
        <i class="bi bi-circle-fill pe-2" style={`color: var(--hot-pink)`}/><small>Total Files & Folders</small>
      </div>
      <div class="text-end"><small>{totalItems}</small></div>
    </div>
    <div class="columns-2">
      <div class="">
        <i class="bi bi-circle-fill pe-2" style={`color: var(--xls-icon)`}/><small>Total Docs</small>
      </div>
      <div class="text-end"><small>{totalDocs}</small></div>
    </div>
    <div class="columns-2">
      <div class="">
        <i class="bi bi-circle-fill pe-2" style={`color: var(--purple)`}/><small>Total Docs Parsed</small>
      </div>
      <div class="text-end"><small>{filesParsed}</small></div>
    </div>
  </div>
</div>

<style lang="scss">
  .bar-segment:hover {
    filter: brightness(1.1) !important;
  }
</style>