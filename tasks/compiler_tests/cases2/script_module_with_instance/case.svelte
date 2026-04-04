<script module>
	import { writable } from 'svelte/store';
	export const theme = writable('light');
</script>

<script>
	let count = $state(0);
</script>

<button onclick={() => count++}>{count}</button>
