<svelte:options runes={false} />
<script>
	var foo = 1;
	export { foo };
	export function getFoo() {
		return foo;
	}
</script>

<p>{foo}</p>
