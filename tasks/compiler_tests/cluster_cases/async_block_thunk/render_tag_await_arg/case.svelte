<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve({ value });
	}
</script>

{#snippet row(value)}<p>{value}</p>{/snippet}

<button onclick={() => x++}>inc</button>

{@render row(await (await delay(x)).value)}
