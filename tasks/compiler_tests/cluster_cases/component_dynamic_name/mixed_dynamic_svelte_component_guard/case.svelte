<script>
	import A from './A.svelte';
	const B = $derived(A);
</script>

<B>
	<svelte:component this={B}>test</svelte:component>
</B>
