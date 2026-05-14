<svelte:options runes={false} />
<script>
	let foo = 1;
	export { foo };
	export function getFoo() {
		return foo;
	}
</script>

<p>{foo}</p>
