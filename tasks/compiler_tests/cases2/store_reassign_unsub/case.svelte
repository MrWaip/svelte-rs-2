<script>
	import { writable } from 'svelte/store';
	let count = writable(0);

	function swap() {
		count = writable(10);
	}
</script>

<p>{$count}</p>
