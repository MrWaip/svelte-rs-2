<script>
	let arr = $state([1, 2, 3]);
	let elements = $state([]);
</script>
{#each arr as item, i (item)}
	<b bind:this={() => elements[i], (v) => elements[i] = v}>{item}</b>
{/each}
