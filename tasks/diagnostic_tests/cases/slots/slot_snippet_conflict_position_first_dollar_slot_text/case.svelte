<script>
	// $$slot is mentioned here first
	let { children } = $props();
</script>

{$$slots}
{@render children?.()}
