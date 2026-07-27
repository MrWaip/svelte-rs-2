<script>
	let a = await Promise.resolve(1);
</script>
<svelte:boundary>
	{@const b = await Promise.resolve(2)}
	{#if b + a > 0}<p>yes</p>{/if}
</svelte:boundary>
