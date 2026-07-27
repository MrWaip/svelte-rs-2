<script>
	import A from './A.svelte';
	import B from './B.svelte';
	let value = $state(0);
	let Comp = $derived(value % 2 === 0 ? A : B);
</script>

<Comp --prop="red" />
