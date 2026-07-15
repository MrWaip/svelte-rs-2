<script>
	import { writable } from 'svelte/store';
	export const name = writable('world');
</script>

<textarea bind:value={$name}></textarea>
<div contenteditable="true" bind:innerHTML={$name}></div>
