<script>
	let v = $state('dog');
</script>

<select>
	<option value={v}>Dog</option>
</select>
<button onclick={() => (v = 'cat')}>swap</button>
