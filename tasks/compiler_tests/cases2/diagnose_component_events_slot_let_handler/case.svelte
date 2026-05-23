<script>
	import Inner from './Inner.svelte';
	import Button from './Button.svelte';
</script>

<Inner let:onClick>
	<Button on:click={onClick} />
</Inner>
