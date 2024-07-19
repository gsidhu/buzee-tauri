<script lang="ts">
	export let filetype: string;
	export let color = true;
	export let extraClasses = '';
</script>

<span class="w-full flex justify-center items-center">
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
	<i class={`bi bi-browser-firefox ${color ? 'pdf-icon' : ''} ${extraClasses}`} title="Firefox Webpage"></i>
{:else if ['chrome-webpage'].includes(filetype)}
	<i class={`bi bi-browser-chrome ${color ? 'excel-icon' : ''} ${extraClasses}`} title="Chrome Webpage"></i>
{:else if ['arc-webpage'].includes(filetype)}
	<!-- <i class={`bi bi-browser-chrome ${color ? 'word-icon' : ''} ${extraClasses}`} title="Webpage"></i> -->
	<i class={`${color ? 'word-icon' : ''} ${extraClasses}`} title="Arc Webpage">
		<!-- <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 49 49" height="12" width="12">
			<path fill="currentColor" opacity="opacity" d="M47.21 17.88c.28-1.36.07-2.79-.64-3.99-.8-1.35-2.1-2.26-3.63-2.57-.35-.07-.72-.11-1.07-.11h-.02c-2.62 0-4.82 1.88-5.34 4.45-.31 1.51-.91 2.97-1.75 4.32a.5.5 0 0 1-.87-.05l-5.44-11.4c-.69-1.45-1.93-2.59-3.47-3.04-2.64-.78-5.36.54-6.48 2.9l-5.62 11.82c-.15.31-.59.3-.72-.01v-.01a5.442 5.442 0 0 0-5.07-3.42c-.72 0-1.43.14-2.11.42-2.8 1.17-4.05 4.47-2.89 7.27 1.06 2.53 2.8 4.92 5.06 6.98.16.15.21.38.12.58l-1.82 3.82c-1.28 2.7-.25 5.99 2.41 7.34.78.39 1.61.59 2.47.59 2.1 0 4.04-1.22 4.94-3.12l1.57-3.29c.1-.22.35-.33.58-.26 1.91.52 3.83.8 5.72.8 2.13 0 4.23-.32 6.26-.91a.5.5 0 0 1 .59.26l1.55 3.26c.92 1.94 2.84 3.26 4.98 3.27.87 0 1.71-.19 2.5-.59 2.67-1.35 3.69-4.65 2.41-7.36l-1.95-4.1c-.09-.2-.05-.43.11-.58 3.78-3.52 6.57-8.13 7.62-13.27zm-33.73 21.9a3.491 3.491 0 0 1-3.15 1.99c-.52 0-1.05-.12-1.54-.36-1.72-.85-2.37-2.99-1.54-4.72l1.51-3.17c.13-.28.49-.38.73-.2.32.23.65.45.99.67 1.33.85 2.72 1.57 4.15 2.16.27.11.39.41.27.67l-1.42 2.96zm9.66-3.89c-8.01 0-16.47-5.43-19.27-12.36A3.48 3.48 0 0 1 5.79 19c.43-.17.87-.26 1.3-.26 1.38 0 2.68.82 3.23 2.18 1.75 4.34 7.62 8.01 12.82 8.01.8 0 1.6-.08 2.39-.22.22-.04.44.07.54.27l2.62 5.52c.13.28-.02.61-.31.69-1.71.45-3.46.7-5.24.7zM22 26.51l1.21-2.54c.1-.2.39-.2.49 0l1.18 2.48c.08.16-.03.36-.2.39-.51.07-1.02.11-1.52.11-.31 0-.62-.02-.93-.05-.21-.03-.31-.23-.23-.39zM38.1 41.4a3.485 3.485 0 0 1-4.69-1.63l-9.49-19.95c-.19-.4-.76-.4-.94 0l-2.94 6.19c-.11.23-.37.34-.61.26-2-.69-3.9-1.86-5.31-3.27a.486.486 0 0 1-.09-.56l6.36-13.38c.22-.47.56-.89.99-1.18 2-1.31 4.34-.49 5.21 1.34l13.05 27.45c.82 1.74.18 3.88-1.54 4.73zm-2.75-18.36a.492.492 0 0 1 .06-.52c1.49-1.91 2.58-4.14 3.05-6.51.33-1.65 1.78-2.8 3.41-2.8.28 0 .56.03.85.1 1.85.46 2.92 2.38 2.53 4.25-.92 4.38-3.2 8.35-6.31 11.46-.25.25-.66.18-.81-.13l-2.78-5.85z"></path>
		</svg> -->
		<svg width="12" height="12" viewBox="0 0 40 35" fill="none" xmlns="http://www.w3.org/2000/svg"><g filter="url(#a)"><path d="m26.906 8.075-1.917-3.947a6.623 6.623 0 0 0-11.958.099l-2.644 5.676a6.617 6.617 0 0 0-6.431-.147c-3.162 1.652-4.271 5.564-2.95 8.602.755 1.747 1.857 3.37 3.207 4.808l-.696 1.495a6.612 6.612 0 0 0 .259 6.089 6.526 6.526 0 0 0 2.409 2.425 6.613 6.613 0 0 0 4.09.86l.02-.003.018-.002a6.7 6.7 0 0 0 5.252-3.874l.348-.747c1.014.163 2.04.244 3.07.241h.028a19.277 19.277 0 0 0 3.483-.36l.435.895.002.004a6.637 6.637 0 0 0 6 3.723h.015a6.63 6.63 0 0 0 6.407-5.204l.005-.025.005-.024a6.706 6.706 0 0 0-.574-4.334l-.781-1.616c2.557-2.793 4.433-6.282 5.17-10.145A6.59 6.59 0 0 0 38.14 7.59a6.63 6.63 0 0 0-5.524-2.891h-.014a6.628 6.628 0 0 0-5.697 3.377Zm-7.27 8.277-.15-.307.15.307Zm-1.078.045.173-.37-.173.37Zm-9.487-3.669a3.522 3.522 0 0 0-3.679-.235 3.523 3.523 0 0 1 3.679.235Zm8.125 6.593ZM6.613 25.334l-.294.632.294-.632Zm25.404.358ZM22.209 5.479Zm7.976 3.342a3.535 3.535 0 0 1 2.455-1.032 3.539 3.539 0 0 1 2.95 1.546 3.498 3.498 0 0 1 .608 2.225l-6.013-2.74Z" fill="#FFFCEA"></path></g><path d="m11.12 15.662-1.546 3.325-1.615 3.47a18.27 18.27 0 0 0 1.914 1.39 18.317 18.317 0 0 0 4.302 1.984l1.56-3.35 1.465-3.15c-2.359-.493-4.72-1.916-6.08-3.67Z" fill="#210784"></path><path d="m30.252 22.063-3.27-6.736a11.892 11.892 0 0 1-2.494 2.306 10.684 10.684 0 0 1-3.459 1.591l3.125 6.42a18.08 18.08 0 0 0 4.312-2.193 18.72 18.72 0 0 0 1.786-1.388Z" fill="#26069C"></path><path d="M9.867 23.844a18.1 18.1 0 0 1-1.914-1.39l-1.634 3.512a3.522 3.522 0 0 0 .14 3.25c.303.535.745.98 1.278 1.285.659.387 1.426.55 2.185.462a3.61 3.61 0 0 0 2.834-2.097l1.413-3.037a18.32 18.32 0 0 1-4.302-1.985Z" fill="#2404AA"></path><path d="M35.59 9.335a3.538 3.538 0 0 0-2.95-1.546 3.537 3.537 0 0 0-3.433 2.877 10.647 10.647 0 0 1-2.22 4.662l3.27 6.735c2.96-2.603 5.134-6.14 5.885-10.081a3.5 3.5 0 0 0-.552-2.647Z" fill="#F99"></path><g filter="url(#b)"><path d="m32.017 25.692-1.758-3.634-3.27-6.735-4.78-9.844a3.53 3.53 0 0 0-6.376.053l-4.717 10.125c1.359 1.754 3.726 3.184 6.079 3.667l1.535-3.298a.412.412 0 0 1 .744-.006l1.562 3.204 3.125 6.42 1.547 3.188a3.547 3.547 0 0 0 3.206 1.99 3.538 3.538 0 0 0 3.42-2.778 3.615 3.615 0 0 0-.317-2.352Z" fill="#FF5060"></path></g><g filter="url(#c)"><path d="M21.033 19.223a9.02 9.02 0 0 1-2.123.276 8.23 8.23 0 0 1-1.712-.178c-2.353-.489-4.715-1.912-6.073-3.665a6.195 6.195 0 0 1-.806-1.337 3.523 3.523 0 0 0-4.927-1.826c-1.616.84-2.28 2.962-1.55 4.635.844 1.955 2.272 3.792 4.12 5.328.608.505 1.247.97 1.915 1.39a18.316 18.316 0 0 0 4.302 1.985c1.552.487 3.169.733 4.795.728a16.19 16.19 0 0 0 5.177-.916l-3.118-6.42Z" fill="#0034FE"></path></g><defs><filter id="a" x="-.531" y="-.59" width="40.819" height="35.658" filterUnits="userSpaceOnUse"><feFlood result="BackgroundImageFix"></feFlood><feGaussianBlur in="BackgroundImageFix" stdDeviation=".495"></feGaussianBlur><feComposite in2="SourceAlpha" operator="in" result="effect1_backgroundBlur_259_65398"></feComposite><feBlend in="SourceGraphic" in2="effect1_backgroundBlur_259_65398" result="shape"></feBlend></filter><filter id="b" x="10.125" y="2.501" width="23.271" height="29.311" filterUnits="userSpaceOnUse"><feFlood result="BackgroundImageFix"></feFlood><feGaussianBlur in="BackgroundImageFix" stdDeviation=".495"></feGaussianBlur><feComposite in2="SourceAlpha" operator="in" result="effect1_backgroundBlur_259_65398"></feComposite><feBlend in="SourceGraphic" in2="effect1_backgroundBlur_259_65398" result="shape"></feBlend></filter><filter id="c" x="2.556" y="11.093" width="22.585" height="16.456" filterUnits="userSpaceOnUse"><feFlood result="BackgroundImageFix"></feFlood><feGaussianBlur in="BackgroundImageFix" stdDeviation=".495"></feGaussianBlur><feComposite in2="SourceAlpha" operator="in" result="effect1_backgroundBlur_259_65398"></feComposite><feBlend in="SourceGraphic" in2="effect1_backgroundBlur_259_65398" result="shape"></feBlend></filter></defs></svg>
	</i>
{:else}
	<i class={`bi bi-file-earmark ${color ? 'txt-icon' : ''} ${extraClasses}`} title="File"></i>
{/if}
</span>

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