<script>
	import Child from './Child.svelte';

	function delay(value) {
		return Promise.resolve(value);
	}

	let loaded = await delay(1);
	let x = $state(0);
</script>

<button onclick={() => x++}>inc</button>

<Child a={loaded} b={await delay(x)} />
