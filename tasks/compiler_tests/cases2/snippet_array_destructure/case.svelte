<script>
	let pair = $state([10, 20]);
</script>

{#snippet show([a, b])}
	<p>{a} and {b}</p>
{/snippet}

{#snippet withRest([first, ...others])}
	<p>{first}</p>
{/snippet}

{@render show(pair)}
{@render withRest([1, 2, 3])}
