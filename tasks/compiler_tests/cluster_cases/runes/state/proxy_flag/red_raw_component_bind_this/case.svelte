<script>
	import Inner from './Inner.svelte';
	let elem = $state.raw();
	$effect(() => {
		console.log(elem);
	});
</script>

<svelte:component this={Inner} bind:this={elem} />
