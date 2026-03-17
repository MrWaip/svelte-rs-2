<script>
	let html = $state('');
	let text = $state('');
	let content = $state('');
</script>

<div contenteditable bind:innerHTML={html}></div>

<div contenteditable bind:innerText={text}></div>

<div contenteditable bind:textContent={content}></div>
