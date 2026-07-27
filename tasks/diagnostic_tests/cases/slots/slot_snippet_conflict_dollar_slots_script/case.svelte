<script>
	export let items;
	const has_footer = $$slots.footer;
</script>

{#snippet view()}
	<p>{items.length} {has_footer}</p>
{/snippet}

{@render view()}
