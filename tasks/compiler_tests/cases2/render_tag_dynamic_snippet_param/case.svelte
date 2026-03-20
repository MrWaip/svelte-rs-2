<script>
	let msg = $state("hi");
</script>

{#snippet wrapper(inner)}
	<div>{@render inner()}</div>
{/snippet}

{#snippet greeting()}
	<p>Hello</p>
{/snippet}

{@render wrapper(greeting)}
