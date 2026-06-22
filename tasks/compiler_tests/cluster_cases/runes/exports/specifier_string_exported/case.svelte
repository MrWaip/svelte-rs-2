<svelte:options runes />
<script>
	let foo = 1;
	export { foo as "foo-bar" };
</script>

<p>{foo}</p>
