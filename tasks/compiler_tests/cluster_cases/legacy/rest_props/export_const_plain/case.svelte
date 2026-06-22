<svelte:options runes={false} />
<script>
	export const foo = 1;
</script>

<div {...$$restProps}>{foo}</div>
