<script>
	let items = $state([1, 2, 3]);
</script>

<svg>
	{#each items as item}
		<circle cx={item * 10} cy={10} r={5} />
	{/each}
</svg>
