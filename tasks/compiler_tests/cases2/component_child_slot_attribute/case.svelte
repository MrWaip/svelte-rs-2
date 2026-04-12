<script>
	import Inner from './Inner.svelte';
	import Outer from './Outer.svelte';
</script>

<Outer>
	<Inner slot="footer" />
</Outer>
