<script>
	import A from './A.svelte';
	const B = $derived(A);
</script>

<B>one</B>
<B>two</B>
