<script>
	/** @type {{ name: string, count?: number }} */
	let { name, count = 0 } = $props();

	/** @type {number} */
	let doubled = $derived(count * 2);

	/** @type {number} */
	let label = $derived.by(() => {
		// format with prefix
		return `${name}: ${doubled}`;
	});
</script>

<p>{label}</p>
