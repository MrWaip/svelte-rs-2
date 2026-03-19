<script>
	class Form {
		name = $state('');
		email = $state('');
		data = $state.raw({});
	}
	let f = new Form();
</script>

<p>{f.name}</p>
