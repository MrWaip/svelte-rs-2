<script>
	let n = $state(1);
</script>

{#snippet row()}
	{@const a = await Promise.resolve(n)}
	<p>{a}</p>
{/snippet}

{@render row()}
<button onclick={() => n++}>go</button>
