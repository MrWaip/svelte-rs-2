<script module>
	import { writable } from 'svelte/store';

	const theme = writable('light');
</script>

<script>
	let count = $state(0);
</script>

<p>{count}</p>
