<script>
	import Inner from './Inner.svelte';

	function tooltip(node) {
		return { destroy() {} };
	}
</script>

<Inner {@attach tooltip} prop="value" />
