<script>let { items, b } = $props();</script>
{#each items as $$async0}
	<b>{$$async0}</b><i>{await b}</i>
{/each}
