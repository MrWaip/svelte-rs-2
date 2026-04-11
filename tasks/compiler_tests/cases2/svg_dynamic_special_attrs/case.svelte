<svelte:options namespace="svg" />

<script>
	let value = $state(1);
	let disabled = false;
</script>

<rect value={value} disabled={disabled} />
