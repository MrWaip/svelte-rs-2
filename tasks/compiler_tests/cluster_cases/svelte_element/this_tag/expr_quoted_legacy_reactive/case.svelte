<script>
	export let n;
	$: tag = 'h' + n;
</script>
<svelte:element this="{tag}">hello</svelte:element>
