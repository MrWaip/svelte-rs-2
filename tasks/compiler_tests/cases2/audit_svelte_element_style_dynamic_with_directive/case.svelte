<script>
	let tag = $state("div");
	let s = $state("color: red");
	let fs = $state("12px");
</script>

<svelte:element this={tag} style={s} style:font-size={fs}>x</svelte:element>
