<script>
	let v = $state('a');
</script>

<select>
	<option value={v}><span>{v}</span></option>
</select>

<button onclick={() => (v = 'b')}>x</button>
