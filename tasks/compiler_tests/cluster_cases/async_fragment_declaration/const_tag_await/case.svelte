<script>
	let n = $state(1);
</script>

{#if n}
	{@const a = await Promise.resolve(n)}
	<p>{a}</p>
{/if}
<button onclick={() => n++}>go</button>
