<script>
	let value = $state('');
	let checked = $state(false);
	let selected = $state(true);
	let disabled = false;
	let readonly = false;
</script>

<input {value} {disabled} />
<input value={value} readonly={readonly} />
<input type="checkbox" checked={checked} />
<option selected={selected}>picked</option>
