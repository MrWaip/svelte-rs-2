<script>
	let p = $state(Promise.resolve([[1, 2], 3]));
</script>
{#await p then [[a, b] = [8, 9], c]}
	<button>{a}{b}{c}</button>
{/await}
