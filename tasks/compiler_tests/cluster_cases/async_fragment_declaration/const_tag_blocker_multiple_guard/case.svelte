<script>
	let n = $state(1);
	let d = $derived(await Promise.resolve(n));
	let e = $derived(await Promise.resolve(n + 1));
</script>

{#if d}
	{@const v = d + e}
	<p>{v}</p>
{/if}
<button onclick={() => n++}>go</button>
