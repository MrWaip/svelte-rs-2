<script>
	let n = $state(1);
</script>

{#if n}
	{@const before = "sync"}
	{@const awaited = await Promise.resolve(n)}
	{@const after = awaited + 1}
	<p>{before}{awaited}{after}</p>
{/if}
<button onclick={() => n++}>go</button>
