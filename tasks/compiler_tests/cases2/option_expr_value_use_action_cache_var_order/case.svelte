<script>
	let value = $state("a");
	function pick() { value = "b"; }
	function action() {}
</script>

<button onclick={pick}>pick</button>
<select>
	<option value={value} use:action>A</option>
</select>
