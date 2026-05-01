<script>
	async function load() {
		return 1;
	}
</script>

{#if true}
	{@const x = load() + await load()}
	<p>{x}</p>
{/if}
