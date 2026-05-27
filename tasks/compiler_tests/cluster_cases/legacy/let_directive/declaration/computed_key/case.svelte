<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
	const k = "z";
</script>
<Inner let:item={{ [k]: v }}>{v}</Inner>
