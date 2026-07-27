<script>
	import Child from './Child.svelte';

	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

<Child value={await delay(x)}>
	{#snippet extra()}{x}{/snippet}
</Child>
