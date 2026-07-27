<script>
	import Child from './Child.svelte';
	let plain = 7;
</script>

{#snippet row()}
	{@const kLit = 'x'}
	{@const kArith = plain + 1}
	<Child {kLit} {kArith} />
{/snippet}

{@render row()}
