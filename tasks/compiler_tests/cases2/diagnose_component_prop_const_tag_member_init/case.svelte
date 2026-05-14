<script>
	import Comp from './Comp.svelte';
	let { p } = $props();
	const store = { sel: { y: 1 } };
</script>

{#if true}
	{@const a = store.sel}
	<Comp foo={a.y} />
{/if}
