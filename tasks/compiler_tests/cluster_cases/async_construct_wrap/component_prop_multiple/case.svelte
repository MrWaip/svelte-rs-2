<script>
	import Child from './Child.svelte';

	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>


<Child a={await delay(x)} b={await delay(x + 1)} />