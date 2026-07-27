<script>
	export let items;
</script>

{$$slots}
<slot />

{#snippet view()}
	<p>{items.length}</p>
{/snippet}

{@render view()}
