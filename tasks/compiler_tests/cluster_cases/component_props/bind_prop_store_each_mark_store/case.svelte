<script>
	import { writable } from 'svelte/store';
	import Child from './Child.svelte';
	const rows = writable([]);
</script>

{#each $rows as row}
	<Child bind:value={row.name} />
{/each}
