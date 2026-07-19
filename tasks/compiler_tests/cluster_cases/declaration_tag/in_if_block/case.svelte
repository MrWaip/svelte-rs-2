<script>
	let { data } = $props();
</script>
{#if data}
	{const foo = data.foo}
	<p>{foo}</p>
{/if}
