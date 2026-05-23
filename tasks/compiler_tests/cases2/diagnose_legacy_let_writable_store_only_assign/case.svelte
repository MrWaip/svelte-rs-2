<svelte:options runes={false} />

<script>
	import { writable } from 'svelte/store';
	let value = writable({ foo: 1 });
	function reset() {
		$value = { foo: 0 };
	}
</script>

<button on:click={reset}>{$value.foo}</button>
