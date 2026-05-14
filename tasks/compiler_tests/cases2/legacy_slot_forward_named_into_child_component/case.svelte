<script>
	import Inner from './Inner.svelte';
</script>

<Inner>
	<slot name="icon" slot="icon" />
	<slot name="caption" slot="caption" />
</Inner>
