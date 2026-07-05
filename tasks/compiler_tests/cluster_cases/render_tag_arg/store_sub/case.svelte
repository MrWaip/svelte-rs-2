<script>
	import { writable } from 'svelte/store';
	let { children } = $props();
	const s = writable(0);
</script>

{@render children($s)}
