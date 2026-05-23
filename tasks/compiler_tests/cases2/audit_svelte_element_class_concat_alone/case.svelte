<script>
	let tag = $state("div");
	let b = $state("two");
	$effect(() => { b = b + ""; });
</script>

<svelte:element this={tag} class="one {b}">x</svelte:element>
