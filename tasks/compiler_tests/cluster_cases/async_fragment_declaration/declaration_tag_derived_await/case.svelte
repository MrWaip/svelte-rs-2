<script>
	let n = $state(1);
</script>

{#if n}
	{let g = $derived(await Promise.resolve(n))}
	<p>{g}</p>
{/if}
<button onclick={() => n++}>go</button>
