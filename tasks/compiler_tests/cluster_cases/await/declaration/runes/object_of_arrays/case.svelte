<script>
	let p = $state(Promise.resolve({ p: [1, 2], q: [3, 4] }));
</script>
{#await p then { p: [a, b], q: [c, d] }}
	<button>{a}{b}{c}{d}</button>
{/await}
