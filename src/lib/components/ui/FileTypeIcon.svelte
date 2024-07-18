<script lang="ts">
	export let filetype: string;
	export let color = true;
	export let extraClasses = '';
</script>

{#if filetype === 'docx'}
	<i class={`bi bi-file-earmark-word-fill ${color ? 'word-icon' : ''} ${extraClasses}`} title="MS Word (.docx)" />
{:else if filetype === 'md'}
	<i class={`bi bi-filetype-md ${color ? 'markdown-icon' : ''} ${extraClasses}`} title="Markdown (.md)" />
{:else if filetype === 'txt'}
	<i class={`bi bi-filetype-txt ${color ? 'txt-icon' : ''} ${extraClasses}`} title="Text (.txt)" />
{:else if filetype === 'csv'}
	<i class={`bi bi-filetype-csv ${color ? 'csv-icon' : ''} ${extraClasses}`} title="CSV (.csv)" />
{:else if filetype === 'pdf'}
	<i class={`bi bi-file-earmark-pdf-fill ${color ? 'pdf-icon' : ''} ${extraClasses}`} title="PDF (.pdf)" />
{:else if filetype === 'pptx'}
	<i class={`bi bi-file-earmark-ppt-fill ${color ? 'powerpoint-icon' : ''} ${extraClasses}`} title="MS PowerPoint (.pptx)" />
{:else if filetype === 'xls' || filetype === 'xlsx'}
	<i class={`bi bi-file-earmark-excel-fill ${color ? 'excel-icon' : ''} ${extraClasses}`} title="MS Excel (.xlsx, .xls)" />
{:else if filetype === 'numbers'}
	<i class={`bi bi-file-earmark-spreadsheet-fill ${color ? 'numbers-icon' : ''} ${extraClasses}`} title="Apple Numbers (.numbers)"></i>
{:else if filetype === 'pages'}
	<i class={`bi bi-file-earmark-richtext-fill ${color ? 'pages-icon' : ''} ${extraClasses}`} title="Apple Pages (.pages)"></i>
{:else if filetype === 'key'}
	<i class={`bi bi-file-earmark-slides-fill ${color ? 'keynote-icon' : ''} ${extraClasses}`} title="Apple Keynote (.key)"></i>
{:else if ['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp'].includes(filetype)}
	<i class={`bi bi-file-earmark-image ${color ? 'image-icon' : ''} ${extraClasses}`} title="Image"></i>
{:else if ['mp3', 'wav', 'flac', 'm4a', 'aac', 'ogg', 'wma', 'aiff'].includes(filetype)}
	<i class={`bi bi-file-earmark-music-fill ${color ? 'audio-icon' : ''} ${extraClasses}`} title="Audio"></i>
{:else if ['mp4', 'mkv', 'avi', 'mov', 'wmv', 'flv', 'webm', 'mpg', 'mpeg', '3gp', '3g2'].includes(filetype)}
	<i class={`bi bi-film ${color ? 'video-icon' : ''} ${extraClasses}`} title="Video"></i>
{:else if ['epub', 'mobi', 'azw3'].includes(filetype)}
	<i class={`bi bi-book-half ${color ? 'video-icon' : ''} ${extraClasses}`} title="Book"></i>
{:else if ['folder'].includes(filetype)}
	<i class={`bi bi-folder-fill ${color ? 'pages-icon' : ''} ${extraClasses}`} title="Folder"></i>
{:else if filetype === 'other-file-folder'}
	<i class={`bi bi-folder-fill ${color ? 'txt-icon' : ''} ${extraClasses}`} title="Folder"></i>
	<i class={`bi bi-file-earmark-image ${color ? 'txt-icon' : ''} ${extraClasses}`} title="Folder"></i>
{:else if ['firefox-webpage'].includes(filetype)}
	<i class={`bi bi-browser-firefox ${color ? 'pdf-icon' : ''} ${extraClasses}`} title="Webpage"></i>
{:else if ['chrome-webpage'].includes(filetype)}
	<i class={`bi bi-browser-chrome ${color ? 'word-icon' : ''} ${extraClasses}`} title="Webpage"></i>
{:else}
	<i class={`bi bi-file-earmark ${color ? 'txt-icon' : ''} ${extraClasses}`} title="File"></i>
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
		color: #37db28;
	}
	.pages-icon {
		color: #fd9920;
	}
	.keynote-icon {
		color: #1e88f5;
	}
	.image-icon {
		color: #83C5BE;
	}
	.audio-icon {
		color: #E07BE0;
	}
	.video-icon {
		color: #E1BC29;
	}
</style>