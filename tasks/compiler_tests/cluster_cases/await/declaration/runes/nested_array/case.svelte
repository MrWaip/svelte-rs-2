<script>
	let p = $state(Promise.resolve([[1, 2], [3, 4]]));
</script>
{#await p then [[a, b], [c, d]]}
	<button>{a}{b}{c}{d}</button>
{/await}
