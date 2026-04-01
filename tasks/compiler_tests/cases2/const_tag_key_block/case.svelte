<script>
	let count = $state(1);
</script>

{#key count}
	{@const doubled = count * 2}
	<p>{doubled}</p>
{/key}
