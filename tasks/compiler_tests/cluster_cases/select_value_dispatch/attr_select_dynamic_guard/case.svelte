<script>
	let v = $state('dog');
</script>

<select value={v}>
	<option value="dog">Dog</option>
	<option value="cat">Cat</option>
</select>
<button onclick={() => (v = 'cat')}>swap</button>
