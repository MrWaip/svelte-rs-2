<svelte:options runes={false} />

<script>
	import Inner from './Inner.svelte';
	import Outer from './Outer.svelte';
	export let value;
</script>

<Outer>
	<Inner slot="footer" x={value} />
</Outer>
