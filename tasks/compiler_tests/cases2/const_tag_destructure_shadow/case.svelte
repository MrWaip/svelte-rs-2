<script>
	let { items } = $props();
	let computed_const = $state('shadow');
</script>

{#each items as item}
	{@const { x, y } = item}
	<p>{x}</p>
{/each}
<p>{computed_const}</p>
