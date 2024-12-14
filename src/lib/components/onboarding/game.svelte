<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import Question from './question.svelte';
	import * as Tooltip from "$lib/components/ui/tooltip";
	import FileTypeIcon from "$lib/components/ui/FileTypeIcon.svelte";
	import PopoverIcon from '../ui/popoverIcon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { trackEvent } from '@aptabase/web';
	import { onMount } from 'svelte';
	let showResults = false;
	let showStats = false;
	let currentQuestionIndex = 0;
	let dbStats: DBStat[] = [];
  let docStats: DBStat[] = [];
	let totalDocs = 0;
	let statPercentage: DBStat[] = [];
	let officePDFCount = 0;
  let allDocCount = 0;
	const officePDFTypes = ['doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'pdf'];
	const allDocTypes = [...officePDFTypes, 'pages', 'numbers', 'key', 'txt', 'md', 'csv'];

	function resetVars() {
		dbStats = [];
		totalDocs = 0;
		statPercentage = [];
		officePDFCount = 0;
	}

	// @ts-ignore
	function handleUpdate(event) {
		questions[currentQuestionIndex].answer = event.detail;
		nextQuestion();
	}

	function nextQuestion() {
		if (currentQuestionIndex < questions.length - 1) {
			currentQuestionIndex += 1;
		} else {
			showResults = true;
			if (dbStats === undefined || dbStats.length === 0) {
				resetVars();
				invoke('get_db_stats').then((res) => {
          //@ts-ignore
          dbStats = res;
					totalDocs = dbStats.reduce((acc, curr) => acc + curr.count, 0);
          // those items from dbStats where file_type is in allDocTypes
          docStats = dbStats.filter((stat) => allDocTypes.indexOf(stat.file_type) > -1);
					dbStats.map((stat) => {
						if (officePDFTypes.indexOf(stat.file_type) > -1) {
							officePDFCount += stat.count;
						}
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
					showStats = true;
					setTimeout(() => {
						document
							.getElementsByClassName('progress-stacked')[0]
							.classList.remove('disable-pointer-events');
					}, 3000);
					console.log(statPercentage);
				});
			}
		}
	}

	let questions = [
		{
			question:
				"Take a guess: What's the approximate number of documents you have on your computer?",
			description: 'Buzee can search over 30,000 documents in less than 1 second!',
			type: 'single-select',
			options: ['Less than 500', 'Maybe 1000', 'At least 5000', 'More than 10,000'],
			answer: ''
		},
		{
			question:
				'Good one. Last question: What percentage of your files do you think are Office Documents (Word, Excel, PowerPoint) and PDFs?',
			description: 'Buzee can search the text inside the document. Even inside PDFs!',
			type: 'range',
			options: [],
			answer: 0
		}
	];

	onMount(() => {
		trackEvent('onboarding:game:start');
	});
</script>

<div in:fly={{ delay: 200, y: 100 }}>
	{#if showResults}
		<div in:fade={{ delay: 200, duration: 1000 }}>
			<h3>Results</h3>
			<p>
				So you said that you have <strong
					>{String(questions[0].answer).toLowerCase()} documents</strong
				>
				and <strong>{questions[1].answer} percent</strong> of them are Word, Excel, PowerPoint or PDF.
			</p>
			<p>Turns out...</p>
			<div in:fade={{ delay: 1000, duration: 1500 }}>
				{#if showStats}
					<p>
						You have over <span class="underline underline-offset-4 decoration-2 decoration-purple">{totalDocs} documents</span> on your computer! Of these,
						<span class="underline underline-offset-4 decoration-2 decoration-purple">{Math.round((100 * officePDFCount) / totalDocs)} percent</span> are MS Office documents
						or PDFs.
					</p>
					<p>Here is your <strong>Unique Document Profile:</strong></p>
					<!-- <div class="progress-stacked my-2 disable-pointer-events">
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
					</div> -->
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
