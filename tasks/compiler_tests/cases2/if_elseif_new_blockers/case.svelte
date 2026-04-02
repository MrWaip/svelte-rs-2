<script>
	const a = await first_fetch();
	const b = await second_fetch();
</script>

{#if a}
	<p>a</p>
{:else if b}
	<p>b</p>
{:else}
	<p>fallback</p>
{/if}
