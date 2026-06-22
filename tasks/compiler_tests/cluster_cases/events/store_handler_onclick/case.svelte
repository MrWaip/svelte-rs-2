<script>
	import { writable } from 'svelte/store';
	const handler = writable();
</script>

<button onclick={$handler}>x</button>
