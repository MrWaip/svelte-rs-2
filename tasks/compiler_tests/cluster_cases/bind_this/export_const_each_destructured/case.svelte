<script>
	export const items1 = {};
	let data = [{ id: 1, text: "a" }];
</script>

{#each data as {id, text} (id)}
	<div bind:this={items1[id]}>{text}</div>
{/each}
