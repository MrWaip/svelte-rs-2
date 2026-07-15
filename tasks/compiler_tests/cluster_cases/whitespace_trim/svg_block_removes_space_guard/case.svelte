<script>
	let { items } = $props();
</script>

<svg>
	{#each items as i}
		<rect></rect>
		<rect></rect>
	{/each}
</svg>
