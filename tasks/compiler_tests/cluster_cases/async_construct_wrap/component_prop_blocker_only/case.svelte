<script>
	import Child from './Child.svelte';

	function delay(value) {
		return Promise.resolve(value);
	}

	let loaded = await delay(1);
</script>

<Child value={loaded} />
