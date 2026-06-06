<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
</script>
<Inner let:item={{ "a-b": ab, "c d": cd }}>{ab}{cd}</Inner>
