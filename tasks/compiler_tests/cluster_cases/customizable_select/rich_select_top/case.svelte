<script>
	let label = $state('hi');
</script>

<select>
	<span class="hdr">{label}</span>
	<option value="a">A</option>
</select>

<button onclick={() => (label = 'bye')}>x</button>
