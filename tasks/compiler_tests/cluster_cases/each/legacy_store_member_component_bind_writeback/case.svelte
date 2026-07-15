<svelte:options runes={false} />
<script>
	import { writable } from 'svelte/store';
	import Child from './Child.svelte';
	const data = writable({ docs: [] });
</script>

{#each $data.docs as doc, idx (idx)}
	<Child bind:document={doc} />
{/each}
