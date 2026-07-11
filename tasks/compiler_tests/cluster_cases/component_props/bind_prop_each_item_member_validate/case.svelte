<script>
	import Child from './Child.svelte';
	let { rows = $bindable() } = $props();
</script>

{#each rows as row, i}
	<Child bind:value={row.name} />
{/each}
