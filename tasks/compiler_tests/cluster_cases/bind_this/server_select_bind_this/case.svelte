<script>
	let ref;
	let val = $state('a');
</script>

<select bind:value={val} bind:this={ref}>
	<option>a</option>
</select>
