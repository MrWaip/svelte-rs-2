<svelte:options runes={false} />
<script>
	export let foo = 1;
</script>

<div {...$$restProps}>{foo}</div>
