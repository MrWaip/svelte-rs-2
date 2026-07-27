<script>
	let n = $state(1);
</script>

{#if n}
	{@const a = await Promise.resolve(n)}
	{let b = $derived(a * 2), c = a + 1}
	<span>{b} {c}</span>
{/if}
