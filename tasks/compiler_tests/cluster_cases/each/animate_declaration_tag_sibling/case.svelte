<script>
	function flip() {}
</script>

{#each items as n (n)}
	{const a = n}
	<div animate:flip={a}></div>
{/each}
