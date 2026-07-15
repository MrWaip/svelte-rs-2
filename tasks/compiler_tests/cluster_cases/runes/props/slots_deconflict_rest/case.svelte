<script>
	let { a, ...rest } = $props();
</script>

<p>{a} {Object.keys(rest)}</p>

{#if $$slots.foo}
	<p>foo exists</p>
{/if}
