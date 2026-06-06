<script>
	let items = $state([{ p: [1, 2], q: [3, 4] }]);
</script>
{#each items as { p: [a, b], q: [c, d] }}
	<button>{a}{b}{c}{d}</button>
{/each}
