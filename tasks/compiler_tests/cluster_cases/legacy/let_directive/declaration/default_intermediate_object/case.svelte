<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={{ p: { a } = {} }}>{a}</Inner>
