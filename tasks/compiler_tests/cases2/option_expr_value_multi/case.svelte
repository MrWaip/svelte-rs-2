<script>
	let value = $state("a");
	function pick() { value = "b"; }
</script>

<button onclick={pick}>pick</button>
<select>
	<option value={value}>A</option>
	<option value="prefix-{value}">B</option>
</select>
