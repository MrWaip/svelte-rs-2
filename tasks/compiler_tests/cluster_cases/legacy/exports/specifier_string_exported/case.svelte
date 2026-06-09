<svelte:options runes={false} />
<script>
	let foo = 1;
	export { foo as "foo-bar" };
</script>

<p>{foo}</p>
