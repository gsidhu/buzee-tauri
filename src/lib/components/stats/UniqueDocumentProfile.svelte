<script lang="ts">
	import FileTypeIcon from "../ui/FileTypeIcon.svelte";
  import PopoverIcon from '../ui/popoverIcon.svelte';
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";

  let dbStats: DBStat[] = [];
  let totalDocs = 0;
  let docStats: DBStat[] = [];
  let allDocCount = 0;
  let statPercentage: DBStat[] = [];
  const officePDFTypes = ['doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'pdf'];
	const allDocTypes = [...officePDFTypes, 'pages', 'numbers', 'key', 'txt', 'md', 'csv'];

  async function updateDBStats() {
    let dbStats: DBStat[] = await invoke('get_db_stats');
    
    totalDocs = dbStats.reduce((acc, curr) => acc + curr.count, 0);
    // dbStats.map((stat) => {
      // statPercentage.push({
      //   'file_type': stat.file_type,
      //   'count': Math.round(100*stat.count/totalDocs)
      // })
    // })
    totalDocs = dbStats.reduce((acc, curr) => acc + curr.count, 0);
    // those items from dbStats where file_type is in allDocTypes
    docStats = dbStats.filter((stat) => allDocTypes.indexOf(stat.file_type) > -1);
    dbStats.map((stat) => {
      if (allDocTypes.indexOf(stat.file_type) > -1) {
        allDocCount += stat.count;
        statPercentage.push({
          file_type: stat.file_type,
          count: Math.round((100 * stat.count) / totalDocs)
        });
      }
    });
    let sumStatPercentages = statPercentage.reduce((acc, curr) => acc + curr.count, 0);
    statPercentage.push({
      file_type: 'other',
      count: 100 - sumStatPercentages
    });
    docStats.push({
      file_type: 'other',
      count: totalDocs - allDocCount
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

<h6 class="text-center">Unique Document Profile</h6>
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
          ? 'jpg'
          : stat.file_type
          }
          filetypeicon={true}
          title={`${stat.count} ${stat.file_type} files`}
        />
      </div>
    </div>
  {/each}
</div>

<div class="col-10 mx-auto my-2">
  <div class="row row-cols-2">
    {#each docStats as stat}
      {#if stat.count > 0}
        <div class="col">
          <div class="row row-cols-2">
            <div class="col">
              <FileTypeIcon filetype={
                stat.file_type === "other"
                    ? 'jpg'
                    : stat.file_type
              }/>
              <small>{stat.file_type}</small>
            </div>
            <div class="col text-end"><small>{stat.count}</small></div>
          </div>
        </div>
      {/if}
    {/each}
    <div class="col">
      <div class="row row-cols-2">
        <div class="col">
          <i class="bi bi-circle-fill pe-2" style={`color: var(--purple)`}/><small>Total</small>
        </div>
        <div class="col text-end"><small>{totalDocs}</small></div>
      </div>
    </div>
  </div>
</div>