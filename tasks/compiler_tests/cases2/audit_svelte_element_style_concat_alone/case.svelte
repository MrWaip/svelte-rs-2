<script>
	let tag = $state("div");
	let c = $state("red");
	$effect(() => { c = c + ""; });
</script>

<svelte:element this={tag} style="color: {c}">x</svelte:element>
