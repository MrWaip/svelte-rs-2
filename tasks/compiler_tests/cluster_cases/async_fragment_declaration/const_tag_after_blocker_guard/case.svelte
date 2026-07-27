<script>
	let n = $state(1);
	let d = $derived(await Promise.resolve(n));
</script>

{#if d}
	{@const a = d + 1}
	{@const b = a + 1}
	<p>{b}</p>
{/if}
<button onclick={() => n++}>go</button>
