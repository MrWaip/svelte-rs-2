<script>
	import Child from './Child.svelte';
	let { cond, aGet, aSet, bGet, bSet } = $props();
</script>

{#if cond}
	<Child bind:value={() => aGet(), aSet} />
{:else}
	<Child bind:value={() => bGet(), bSet} />
{/if}
