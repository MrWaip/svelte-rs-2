<script>
	import Comp from './Comp.svelte';
	let value = $state({ name: 'a' });
</script>

{#if true}
	{@const item = value}
	<Comp name={item.name} />
{/if}
