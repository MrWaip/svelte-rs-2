<script>
	import { writable } from 'svelte/store';
	import { count } from './stores';
	let state = writable(0);
</script>
<p>{$count}</p>
<p>{$state}</p>
