<script lang="ts">
	import FileTypeIcon from "../ui/FileTypeIcon.svelte";
  import PopoverIcon from '../ui/popoverIcon.svelte';
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";

  let dbStats: DBStat[] = [];
  let totalItems = 0;
  let totalDocs = 0;
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
<div class="progress-stacked w-90 mx-auto">
  {#each docStats as stat, index}
    <div
      class="progress"
      role="progressbar"
      aria-label="One"
      aria-valuenow="10"
      aria-valuemin="0"
      aria-valuemax="100"
      style={`width: ${statPercentage[index].count}%`}
    >
      <div
        class="progress-bar"
        style={stat.file_type === "other"
          ? `background-color: #E15554;`
          : `background-color: var(--${stat.file_type}-icon);`}
      >
        <PopoverIcon
          icon={stat.file_type === "other"
          ? 'other-file-folder'
          : stat.file_type
          }
          filetypeicon={true}
          title={`${stat.count} ${stat.file_type} files (${statPercentage[index].count}%)`}
        />
      </div>
    </div>
  {/each}
</div>

<div class="w-4/5 pr-4 pl-4 mx-auto my-4 cursor-default">
  <div class="row row-cols-sm-2 row-cols-1">
    {#each docStats as stat}
      {#if stat.count > 0}
        <div class="col">
          <div class="row row-cols-2">
            <div class="col-8">
              <FileTypeIcon filetype={
                stat.file_type === "other"
                    ? 'folder'
                    : stat.file_type
              }/>
              <small>{stat.file_type}</small>
            </div>
            <div class="col-4 text-end"><small>{stat.count}</small></div>
          </div>
        </div>
      {/if}
    {/each}
    <div class="col">
      <div class="row row-cols-2">
        <div class="col-8">
          <i class="bi bi-circle-fill pe-2" style={`color: var(--purple)`}/><small>Total Docs</small>
        </div>
        <div class="col-4 text-end"><small>{totalDocs}</small></div>
      </div>
    </div>
    <div class="col">
      <div class="row row-cols-2">
        <div class="col-8">
          <i class="bi bi-circle-fill pe-2" style={`color: var(--hot-pink)`}/><small>Total Files</small>
        </div>
        <div class="col-4 text-end"><small>{totalItems}</small></div>
      </div>
    </div>
  </div>
</div>