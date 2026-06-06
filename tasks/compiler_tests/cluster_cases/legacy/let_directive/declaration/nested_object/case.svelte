<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={{ p: { a }, q: { b } }}>{a}{b}</Inner>
