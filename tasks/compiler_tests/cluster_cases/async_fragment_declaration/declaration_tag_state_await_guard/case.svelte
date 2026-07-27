<script>
	let n = $state(1);
</script>

{#if n}
	{let s = $state(await Promise.resolve(n))}
	<p>{s}</p>
	<button onclick={() => s++}>bump</button>
{/if}
<button onclick={() => n++}>go</button>
