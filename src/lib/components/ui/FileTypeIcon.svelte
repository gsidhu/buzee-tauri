<script lang="ts">
	export let filetype: string;
	export let color = true;
</script>

{#if filetype === 'docx'}
	<i class={`bi bi-file-earmark-word-fill ${color ? 'word-icon' : ''}`} title="MS Word (.docx)" />
{:else if filetype === 'md'}
	<i class={`bi bi-filetype-md ${color ? 'markdown-icon' : ''}`} title="Markdown (.md)" />
{:else if filetype === 'txt'}
	<i class={`bi bi-filetype-txt ${color ? 'txt-icon' : ''}`} title="Text (.txt)" />
{:else if filetype === 'csv'}
	<i class={`bi bi-filetype-csv ${color ? 'csv-icon' : ''}`} title="CSV (.csv)" />
{:else if filetype === 'pdf'}
	<i class={`bi bi-file-earmark-pdf-fill ${color ? 'pdf-icon' : ''}`} title="PDF (.pdf)" />
{:else if filetype === 'pptx'}
	<i class={`bi bi-file-earmark-ppt-fill ${color ? 'powerpoint-icon' : ''}`} title="MS PowerPoint (.pptx)" />
{:else if filetype === 'xls' || filetype === 'xlsx'}
	<i class={`bi bi-file-earmark-excel-fill ${color ? 'excel-icon' : ''}`} title="MS Excel (.xlsx, .xls)" />
{:else if filetype === 'numbers'}
	<i class={`bi bi-file-earmark-spreadsheet-fill ${color ? 'numbers-icon' : ''}`} title="Apple Numbers (.numbers)"></i>
{:else if filetype === 'pages'}
	<i class={`bi bi-file-earmark-richtext-fill ${color ? 'pages-icon' : ''}`} title="Apple Pages (.pages)"></i>
{:else if filetype === 'key'}
	<i class={`bi bi-file-earmark-slides-fill ${color ? 'keynote-icon' : ''}`} title="Apple Keynote (.key)"></i>
{/if}

<style>
	.excel-icon {
		color: #146c43;
	}
	.powerpoint-icon {
		color: #ee6c45;
	}
	.pdf-icon {
		color: #dc3545;
	}
	.csv-icon {
		color: #146c43;
	}
	.txt-icon {
		color: inherit;
	}
	.markdown-icon,
	.word-icon {
		color: #0d6efd;
	}
	.numbers-icon {
		color: #63ee56;
	}
	.pages-icon {
		color: #fd9920;
	}
	.keynote-icon {
		color: #1e88f5;
	}
</style>