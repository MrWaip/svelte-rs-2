<script>
	let { value = "a" } = $props();
</script>

<input type="radio" bind:group={value} value="a">
