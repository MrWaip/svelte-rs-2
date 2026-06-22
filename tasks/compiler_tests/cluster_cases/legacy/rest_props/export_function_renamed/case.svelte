<svelte:options runes={false} />
<script>
	function foo() {}
	export { foo as bar };
</script>

<div {...$$restProps}>{foo()}</div>
