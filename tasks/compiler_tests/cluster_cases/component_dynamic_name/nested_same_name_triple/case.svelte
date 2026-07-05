<script>
	import A from './A.svelte';
	const B = $derived(A);
</script>

<B>
	<B>
		<B>test</B>
	</B>
</B>
