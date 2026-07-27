<script>
	let loaded = await Promise.resolve(1);
	let value = "";

	function getValue() {
		return loaded + value;
	}

	function setValue(v) {
		value = v;
	}
</script>

<input value={getValue()} oninput={(e) => setValue(e.target.value)} />
