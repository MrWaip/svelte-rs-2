<script>
	let items = $state([1, 2]);
</script>

{#each items as item}
	{@debug item}
	<button onclick={() => items.push(0)}>{item}</button>
{/each}
