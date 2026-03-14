<script>
	let { title = "world" } = $props();
	let message = $state("hello");
</script>

{#snippet greeting(name)}
	<p>Hello {name}</p>
{/snippet}

{@render greeting(message)}
{@render greeting(title)}
