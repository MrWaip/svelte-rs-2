<script>
	let items = [1, 2, 3];
</script>

{#each items as item}
	{@const doubled = item * 2}
	<p>{doubled}</p>
{/each}
