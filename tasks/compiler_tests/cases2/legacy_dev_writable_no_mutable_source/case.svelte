<script>
	import { writable } from 'svelte/store';
	let total = writable(0);
</script>
<p>{$total}</p>
