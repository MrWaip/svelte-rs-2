<svelte:options runes={false} />
<script>
	import { writable } from 'svelte/store';
	let foo = writable(42);
	export let x = $foo + 1;
</script>
<p>{x}</p>
