<script>
	export let step;
	$: maxLength = step.maxLength ?? Infinity;
	function noop() {}
</script>
