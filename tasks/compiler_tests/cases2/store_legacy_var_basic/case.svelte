<script>
	import { writable } from 'svelte/store';
	var count = writable(0);
</script>
<p>{$count}</p>
