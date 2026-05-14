<script>
	let { items = [] } = $props();
</script>

{#each items as item, i (i)}
	<div>{i}</div>
{/each}
