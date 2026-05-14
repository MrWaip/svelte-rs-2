<script>
	let tag = $state("div");
	let cls = $state("a b");
	let active = $state(false);
</script>

<svelte:element this={tag} class={cls} class:active>x</svelte:element>
