<script>
	let count = 0;
	const promise = Promise.resolve(1);
</script>

{#if count >= 0}
	{@const value = await promise}
	<p>{value} {count}</p>
{/if}
<button onclick={() => count++}>inc</button>
