<script>
	let v = $state('dog');
</script>

<select>
	<option>{v}</option>
</select>
<button onclick={() => (v = 'cat')}>swap</button>
