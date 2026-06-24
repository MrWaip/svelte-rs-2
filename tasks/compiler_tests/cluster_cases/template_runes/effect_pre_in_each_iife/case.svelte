<script>
	let items = $state([1, 2, 3]);
</script>

{#each items as item (item)}
	{(() => {
		$effect.pre(() => {
			items;
		});
	})()}
	<div>{item}</div>
{/each}
