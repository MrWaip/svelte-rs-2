<script>
	import A from './A.svelte';
	const B = $derived(A);
</script>

<svelte:component this={B}>
	<svelte:component this={B}>test</svelte:component>
</svelte:component>
