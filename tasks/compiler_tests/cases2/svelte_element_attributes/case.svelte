<script>
	let tag = $state("div");
	let myId = $state("foo");
</script>

<svelte:element this={tag} class="bar" id={myId}>content</svelte:element>
