<script>
	let data = await fetch('/api');
</script>

{#if true}
	{@const value = data.text}
	<p>{value}</p>
{/if}
