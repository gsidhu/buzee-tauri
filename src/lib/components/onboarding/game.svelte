<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import Question from './question.svelte';
  import PopoverIcon from '../ui/popoverIcon.svelte';
  import ConfettiButton from '../ui/confettiButton.svelte';
  import { goto } from '$app/navigation';
  let showResults = false;
  let showStats = false;
  let currentQuestionIndex = 0;
  let dbStats: DBStat[] = [];
  let totalDocs = 0;
  let statPercentage: DBStat[] = [];
  let officePDFCount = 0;
  const officePDFTypes = ['doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'pdf'];

  function resetVars() {
    dbStats = [];
    totalDocs = 0;
    statPercentage = [];
    officePDFCount = 0;
  }

  // @ts-ignore
  function handleUpdate(event) {
    console.log('handleUpdate', event.detail);
    questions[currentQuestionIndex].answer = event.detail;
    nextQuestion();
  }

  function nextQuestion() {
    console.log('nextQuestion', currentQuestionIndex);
    if (currentQuestionIndex < questions.length - 1) {
      currentQuestionIndex += 1;
    } else {
      showResults = true;
      if (dbStats === undefined || dbStats.length === 0) {
        resetVars();
        window.dbAPI?.docsCount().then((res: DBStat[]) => {
          dbStats = res;
          totalDocs = dbStats.reduce((acc, curr) => acc + curr.count, 0);
          dbStats.map((stat) => {
            if (officePDFTypes.indexOf(stat.type) > -1) {
              officePDFCount += stat.count;
            }
            statPercentage.push({
              'type': stat.type,
              'count': Math.round(100*stat.count/totalDocs)
            })
          })
          showStats = true;
          setTimeout(() => {
            document.getElementsByClassName('progress-stacked')[0].classList.remove('disable-pointer-events');
          }, 3000);
          console.log(statPercentage);
        });
      }
    }
  }

  let questions = [
    {
      question: "Take a guess: What's the approximate number of documents you have on your computer?",
      description: "Buzee can search over 30,000 documents in less than 1 second!",
      type: "single-select",
      options: ["Less than 500", "Maybe 1000", "At least 5000", "More than 10,000"],
      answer: "",
    },
    {
      question: "Good one. Last question: What percentage of your files do you think are Office Documents (Word, Excel, PowerPoint) and PDFs?",
      description: "Buzee can search the text inside the document. Even inside PDFs!",
      type: "range",
      options: [],
      answer: 0,
    },
  ];
</script>

<div in:fly={{ delay: 200, y: 100 }}>
  {#if showResults}
    <div in:fade={{ delay: 200, duration: 1000 }}>
      <h3>Results</h3>
      <p>So you said that you have <strong>{String(questions[0].answer).toLowerCase()} documents</strong> and <strong>{questions[1].answer} percent</strong> of them are Word, Excel, PowerPoint or PDF.</p>
      <p>Turns out...</p>
      <div in:fade={{ delay: 2000, duration: 1500 }}>
        {#if showStats}
          <p>You have over <mark>{totalDocs} documents</mark> on your computer! Of these, <mark>{Math.round(100*officePDFCount/totalDocs)} percent</mark> are MS Office documents or PDFs.</p>
          <p>Here is your <strong>Unique Document Profile:</strong></p>
          <div class="progress-stacked my-2 disable-pointer-events">
            {#each dbStats as stat, index}
              <div class="progress" role="progressbar" aria-label="One" aria-valuenow="10" aria-valuemin="0" aria-valuemax="100" style={`width: ${statPercentage[index].count}%`}>
                <div class="progress-bar" style={`background-color: var(--${stat.type}-icon)`}>
                  <PopoverIcon icon={stat.type} filetypeicon={true} title={`${stat.count} ${stat.type} files`} />
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <h3>How well do you know your workspace?</h3>
    <Question question={questions[currentQuestionIndex]} on:update={handleUpdate} />
  {/if}
</div>

<style>
  .disable-pointer-events {
    pointer-events: none;
  }
</style>