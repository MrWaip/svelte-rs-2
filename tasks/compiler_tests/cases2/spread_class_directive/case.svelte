<script>
	let props = $state({ id: "a", class: "from-spread" });
	let active = $state(true);
</script>

<div {...props} class:active={active} />
