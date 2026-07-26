<script>
	let n = $state(1);
</script>

{#if n}
	{let outer = $derived(await Promise.resolve(n))}
	<div>
		{const inner = $derived(`v${outer}`)}
		<span>{inner}</span>
	</div>
{/if}
<button onclick={() => n++}>go</button>
