<svelte:options runes={false} />
<script>
	export let { a: x, b: y } = { a: 1, b: 2 };
</script>
<button>{x}{y}</button>
