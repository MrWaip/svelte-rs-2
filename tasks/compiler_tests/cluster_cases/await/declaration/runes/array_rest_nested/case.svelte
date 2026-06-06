<script>
	let p = $state(Promise.resolve([1, 2, 3]));
</script>
{#await p then [a, ...[b, c]]}
	<button>{a}{b}{c}</button>
{/await}
