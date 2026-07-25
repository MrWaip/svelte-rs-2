<script>
	let value = $state('a');
	function onchange() {}
</script>

<select id="s" bind:value {onchange} onfocus={() => value = 'b'} class="x">
	<option value="a">a</option>
</select>
