<script>
	let { a, ...rest } = $props();
</script>
<button>{a}{rest.b}</button>
