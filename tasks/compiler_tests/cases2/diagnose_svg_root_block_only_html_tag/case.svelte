<script>
	let cond = true;
	let raw = '<circle r={5}/>';
</script>

{@html raw}
{#if cond}
	<g><path d="M1"/></g>
{/if}
