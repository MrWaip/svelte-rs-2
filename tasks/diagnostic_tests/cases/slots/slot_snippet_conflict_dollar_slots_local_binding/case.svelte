<script>
	export let items;
	function pick($$slots) {
		return $$slots;
	}
</script>

<p>{pick(items)}</p>

{#snippet view()}
	<p>ok</p>
{/snippet}

{@render view()}
