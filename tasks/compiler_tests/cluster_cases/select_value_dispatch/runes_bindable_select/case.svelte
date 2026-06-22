<script>
	let { selected = $bindable() } = $props();
</script>

<select bind:value={selected}>
	<option>a</option>
	<option>b</option>
</select>
