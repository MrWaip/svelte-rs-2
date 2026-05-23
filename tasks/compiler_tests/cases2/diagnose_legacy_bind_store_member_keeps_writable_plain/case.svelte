<svelte:options runes={false} />

<script>
	import { writable } from 'svelte/store';
	import Child from './Child.svelte';
	export let config;
	let address = writable(Object.assign({}, config.address));
	function reset() {
		$address = { foo: 0 };
	}
</script>

<Child bind:value={$address.foo} />
<button on:click={reset}>{$address.foo}</button>
