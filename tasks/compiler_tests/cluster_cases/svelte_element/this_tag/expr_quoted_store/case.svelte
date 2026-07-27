<script>
	import { writable } from 'svelte/store';
	const tag = writable('div');
</script>
<svelte:element this="{$tag}">hello</svelte:element>
