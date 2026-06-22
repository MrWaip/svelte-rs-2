<script>
	const items1 = {};
	let data = [{ id: 1, text: "a" }];
</script>

{#each data as item (item.id)}
	<div bind:this={items1[item.id]}>{item.text}</div>
{/each}
