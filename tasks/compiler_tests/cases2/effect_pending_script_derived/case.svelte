<script>
	let pending = $derived.by(() => $effect.pending());
</script>

<p>{pending}</p>
