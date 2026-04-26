<svelte:options runes={false} />
<script>
	let foo = 1;
	export { foo };

	function makeBar() {
		return 42;
	}
	let bar = makeBar();
	export { bar };
</script>

<p>{foo}{bar}</p>
