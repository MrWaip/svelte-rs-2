<script>
	let show = true;
	let x = $state(42);
	let items = [1, 2, 3];
</script>

{#if show}
	{@debug x}
{/if}

{#each items as item}
	{@debug item}
{/each}

<div>
	{@debug x}
	<p>Value: {x}</p>
</div>
