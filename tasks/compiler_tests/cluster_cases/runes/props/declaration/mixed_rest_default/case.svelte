<script>
	let { a, b = 2, ...rest } = $props();
</script>
<button>{a}{b}{rest.c}</button>
