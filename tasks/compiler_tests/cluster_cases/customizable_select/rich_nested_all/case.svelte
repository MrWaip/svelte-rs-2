<script>
	let fruit = $state('apple');
</script>

<select>
	<optgroup label="Fruits">
		<span class="hdr">{fruit}</span>
		<option value="a"><span>{fruit}</span> {fruit}</option>
		<option value="b">banana</option>
	</optgroup>
</select>

<button onclick={() => (fruit = 'orange')}>x</button>
