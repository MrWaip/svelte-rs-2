<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={[a, ...[b, c]]}>{a}{b}{c}</Inner>
