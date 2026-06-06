<svelte:options runes={false} />
<script>
	export let { a, ...rest } = { a: 1, b: 2, c: 3 };
</script>
<button>{a}{rest.b}</button>
