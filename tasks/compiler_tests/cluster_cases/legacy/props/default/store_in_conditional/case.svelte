<svelte:options runes={false} />
<script>
	import { writable } from 'svelte/store';
	let foo = writable(42);
	let cond = true;
	export let x = cond ? $foo : 0;
</script>
<p>{x}</p>
