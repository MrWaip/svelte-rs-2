<script>
	import Inner from './Inner.svelte';
	let counter = $state(0);
</script>

<Inner>
	default text
	<div slot="footer">Footer: {counter}</div>
</Inner>
