<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve({ value });
	}
</script>

<button onclick={() => x++}>inc</button>

{#each [1] as item}
	{@const current = (await delay(x)).value}
	<p>{current}{item}</p>
{/each}
