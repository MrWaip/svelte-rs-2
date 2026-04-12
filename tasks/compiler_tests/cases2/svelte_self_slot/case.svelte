<script>
	import Outer from './Outer.svelte';
</script>

<Outer>
	<svelte:self slot="footer" />
</Outer>
