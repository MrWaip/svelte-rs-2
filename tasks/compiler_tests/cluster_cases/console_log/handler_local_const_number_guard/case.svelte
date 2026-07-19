<script>
	let items = $state([1]);
</script>

{#each items as item, i}
	<button onclick={(e) => {
		const index = Number(e.currentTarget.dataset.index);
		console.log(index);
	}}>{item}</button>
{/each}
