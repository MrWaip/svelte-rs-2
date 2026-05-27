<script>
	let count = $state(1);
	function inc() { count++; }
</script>

<button onclick={inc}>+</button>
<select>
	<option value={`x-${count}`}>A</option>
</select>
