<script>
	let query = $state('');
	let name = $state('');
	function upd() {
		query = 'a';
		name = 'b';
	}
</script>

<button onclick={upd}>x</button>
<input value={name} />
<textarea>{query}</textarea>
