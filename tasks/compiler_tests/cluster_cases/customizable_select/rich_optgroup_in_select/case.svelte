<script>
	let label = $state('hi');
</script>

<select>
	<optgroup label="g">
		<em class="hdr">{label}</em>
		<option value="a">A</option>
	</optgroup>
</select>

<button onclick={() => (label = 'bye')}>x</button>
