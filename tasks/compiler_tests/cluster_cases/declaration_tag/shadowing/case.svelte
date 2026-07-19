<script>
	let { value } = $props();
</script>
{#each [1, 2] as n}
	{const value = n * 10}
	<p>{value}</p>
{/each}
<span>{value}</span>
