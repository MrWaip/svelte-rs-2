<script>
	let p = $state(Promise.resolve([1, 2, 3]));
</script>
{#await p then [a, , c]}
	<button>{a}{c}</button>
{/await}
