<script>
	export const items1 = {};
	export let data = [{ id: 1, text: "b" }];
</script>

{#each data as item (item.id)}
	<div bind:this={items1[item.id]}>{item.text}</div>
{/each}
