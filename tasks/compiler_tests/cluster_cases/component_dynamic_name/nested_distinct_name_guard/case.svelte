<script>
	import A from './A.svelte';
	const B = $derived(A);
	const C = $derived(A);
</script>

<B>
	<C>test</C>
</B>
