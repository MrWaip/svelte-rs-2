<script>
	let { header, footer } = $props();
</script>

<select>{@render header()}<div>x</div>{@render footer()}</select>
