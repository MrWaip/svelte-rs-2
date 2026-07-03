<script>
	let { items } = $props();
</script>

<select>{#each items as item}<div>{item}</div>{/each}</select>
