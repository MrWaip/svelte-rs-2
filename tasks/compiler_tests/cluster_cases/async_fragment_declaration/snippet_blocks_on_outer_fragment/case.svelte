<script>
	let n = $state(1);
</script>

{let number = $derived(await Promise.resolve(n))}

{#snippet row()}
	{@const doubled = number * 2}
	<span>{doubled}</span>
{/snippet}

{@render row()}
