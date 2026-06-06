<script>
	let v = $state('x');
</script>

<select>
	<option value="a"><span>{v}</span></option>
	<option value="b">B</option>
</select>

<button onclick={() => (v = 'y')}>x</button>
