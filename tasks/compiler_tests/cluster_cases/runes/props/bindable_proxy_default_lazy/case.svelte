<script>
	const DEFAULTS = { a: 1 };
	let { config = $bindable(DEFAULTS) } = $props();
</script>

<button>{config.a}</button>
