<script>
	let items = $state([1, 2]);
</script>

{#each items as item}
	{@const loaded = await Promise.resolve(item)}
	<p>{loaded}</p>
{/each}
<button onclick={() => items.push(3)}>go</button>
