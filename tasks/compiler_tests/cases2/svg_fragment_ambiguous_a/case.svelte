<script>
	let items = $state([1]);
</script>

<svg>
	{#each items as item}
		<a href="/node-{item}" />
	{/each}
</svg>
