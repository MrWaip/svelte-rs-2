<script>
	let items = $state([1, 2, 3]);
</script>

{#each items, rank}
	<div>{rank}</div>
{/each}
