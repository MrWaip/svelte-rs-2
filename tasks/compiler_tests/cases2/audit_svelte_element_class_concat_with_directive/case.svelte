<script>
	let tag = $state("div");
	let b = $state("two");
	let active = $state(false);
	$effect(() => { b = b + ""; });
</script>

<svelte:element this={tag} class="one {b}" class:active>x</svelte:element>
