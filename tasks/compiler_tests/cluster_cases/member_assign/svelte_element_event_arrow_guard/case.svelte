<script>
	let obj = $state({ x: null });
	let src = $state({});
</script>

<svelte:element this={"button"} onclick={() => (obj.x = src)}>go</svelte:element>
