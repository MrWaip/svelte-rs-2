<script>
	let items = $state(["a", "b"]);
</script>

{#each items as item, index}
	<span>{index}: {item}</span>
{/each}
