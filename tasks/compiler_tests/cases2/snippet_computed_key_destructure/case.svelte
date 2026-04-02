<script>
	let data = $state({
		label: "world",
		extra: "ok"
	});

	function key() {
		return "label";
	}
</script>

{#snippet view({ [key()]: value, ...rest })}
	<p>{value} {rest.extra}</p>
{/snippet}

{@render view(data)}
