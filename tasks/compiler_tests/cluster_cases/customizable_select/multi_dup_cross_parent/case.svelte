<script>
	let { r } = $props();
</script>

<select>{@render r()}</select>
<select><optgroup label="g">{@render r()}</optgroup></select>
<select><option>{@render r()}</option></select>
