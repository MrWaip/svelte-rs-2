<script>
	function delay(value) {
		return Promise.resolve(value);
	}

	let attrs = await delay({ title: 'hi' });
</script>

<div {...attrs}></div>
