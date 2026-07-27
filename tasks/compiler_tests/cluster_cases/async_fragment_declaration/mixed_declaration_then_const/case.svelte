<script>
	let n = $state(1);
</script>

{#if n}
	{let a = $derived(await Promise.resolve(n))}
	{@const b = a * 2}
	<span>{b}</span>
{/if}
