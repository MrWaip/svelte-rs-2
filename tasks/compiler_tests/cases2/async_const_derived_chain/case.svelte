<script>
	let data = await fetch('/api');
</script>

{#if true}
	{@const a = data.value}
	{@const b = a * 2}
	{@const c = b + 1}
	<p>{c}</p>
{/if}
