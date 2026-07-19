<script>
	let { a, b = a, c = b * b } = $props();
</script>

<p>{a}{b}{c}</p>
