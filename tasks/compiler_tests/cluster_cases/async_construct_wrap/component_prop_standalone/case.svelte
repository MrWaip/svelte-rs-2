<script>
	import Child from './Child.svelte';

	let { x } = $props();

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<Child value={await delay(x)} />
