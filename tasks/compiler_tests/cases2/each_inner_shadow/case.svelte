<script>
	let { item, items = [] } = $props();
</script>

{#each items as item}
	<span>{item}</span>
{/each}
