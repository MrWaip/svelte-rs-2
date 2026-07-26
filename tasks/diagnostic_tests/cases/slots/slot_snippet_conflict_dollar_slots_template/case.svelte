<script>
	let { children } = $props();
</script>

{$$slots}
{@render children?.()}
