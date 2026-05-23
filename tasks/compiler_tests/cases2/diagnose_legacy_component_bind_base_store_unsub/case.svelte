<script>
	import { writable } from 'svelte/store';
	import Child from './Child.svelte';

	let value = writable(undefined);

	$: x = $value;
</script>

<Child bind:value />
