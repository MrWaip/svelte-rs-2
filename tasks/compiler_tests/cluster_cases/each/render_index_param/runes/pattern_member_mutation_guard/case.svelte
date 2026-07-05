<script>
	let items = $state([{ obj: { x: 0 } }]);
</script>

{#each items as { obj }}
	<button onclick={() => obj.x++}>{obj.x}</button>
{/each}
