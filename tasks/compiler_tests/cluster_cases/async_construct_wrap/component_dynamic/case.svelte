<script>
	import Child from './Child.svelte';

	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>


<svelte:component this={Child} value={await delay(x)} />