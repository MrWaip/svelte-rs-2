<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={[[a, b] = [8, 9], c]}>{a}{b}{c}</Inner>
