<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={{ a: x, b: y }}>{x}{y}</Inner>
