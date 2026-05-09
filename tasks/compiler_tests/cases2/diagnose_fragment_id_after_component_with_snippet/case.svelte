<script>
	import A from './A.svelte';
	import B from './B.svelte';
	let data = $state(null);
	let x = $state(null);
</script>

<A>
	{#snippet inner()}
		{x}
	{/snippet}
</A>

<B>
	{#if data}
		<div>c</div>
	{/if}
</B>
