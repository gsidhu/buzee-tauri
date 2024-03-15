<script lang="ts">
	import FileTypeIcon from "../ui/FileTypeIcon.svelte";
  import PopoverIcon from '../ui/popoverIcon.svelte';
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";

  let dbStats: DBStat[] = [];
  let totalDocs = 0;
  let statPercentage: DBStat[] = [];

  async function updateDBStats() {
    let dbStats: DBStat[] = await invoke('get_db_stats');
    
    totalDocs = dbStats.reduce((acc, curr) => acc + curr.count, 0);
    dbStats.map((stat) => {
      statPercentage.push({
        'file_type': stat.file_type,
        'count': Math.round(100*stat.count/totalDocs)
      })
    })

    console.log("stats:", statPercentage);
    console.log("counts:", dbStats);
  }

  // function saveUniqueDocumentProfile() {
  //   console.log('saveUniqueDocumentProfile');
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
  {#each dbStats as stat, index}
    {stat}
    <div class="progress" role="progressbar" aria-label={stat.file_type} aria-valuenow="10" aria-valuemin="0" aria-valuemax="100" style={`width: ${statPercentage[index].count}%`}>
      <div class="progress-bar" style={`background-color: var(--${stat.file_type}-icon)`}>
        <FileTypeIcon filetype={stat.file_type} color={false}/>
        <!-- <PopoverIcon icon={stat.file_type} filetypeicon={true} title={`${stat.count} ${stat.file_type} files`} /> -->
      </div>
    </div>
  {/each}
</div>

{#each dbStats as stat}
<div>{stat.file_type}</div>
{/each}


<div class="col-10 mx-auto my-2">
  <div class="row row-cols-2">
    {#each dbStats as stat}
      {#if stat.count > 0}
        <div class="col">
          <div class="row row-cols-2">
            <div class="col">
              <FileTypeIcon filetype={stat.file_type}/>
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