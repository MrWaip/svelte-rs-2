<svelte:options runes={false} />
<script>
	export let { p: { a }, q: { b } } = { p: { a: 1 }, q: { b: 2 } };
</script>
<button>{a}{b}</button>
