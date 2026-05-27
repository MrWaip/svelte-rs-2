<script>
	let items = $state([1, 2, 3]);
</script>

{#each items as item, i}
	<button onclick={i}>{item}</button>
{/each}
