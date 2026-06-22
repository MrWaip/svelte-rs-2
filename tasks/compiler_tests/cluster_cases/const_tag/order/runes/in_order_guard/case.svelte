<script>
	let n = $state('world');
</script>
{#if n}
	{@const bar = n}
	{@const foo = bar}
	<p>{foo}</p>
{/if}
