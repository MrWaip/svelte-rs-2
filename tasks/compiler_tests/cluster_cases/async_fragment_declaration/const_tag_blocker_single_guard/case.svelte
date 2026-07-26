<script>
	let n = $state(1);
	let d = $derived(await Promise.resolve(n));
</script>

{#if d}
	{@const v = d + 1}
	<p>{v}</p>
{/if}
<button onclick={() => n++}>go</button>
