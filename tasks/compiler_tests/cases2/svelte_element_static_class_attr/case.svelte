<script>
	let tag = $state('div');
	let title = $state('hello');
</script>

<svelte:element this={tag} class="my-class">
	Content: {title}
</svelte:element>
