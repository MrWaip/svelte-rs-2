<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={[a = 10, b = 20]}>{a}{b}</Inner>
