<script>
	let { format = (x) => x, value = $bindable(), extra } = $props();
</script>

<span>{format(value)}{extra}</span>
