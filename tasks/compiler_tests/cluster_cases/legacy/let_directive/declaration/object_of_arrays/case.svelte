<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={{ p: [a, b], q: [c, d] }}>{a}{b}{c}{d}</Inner>
