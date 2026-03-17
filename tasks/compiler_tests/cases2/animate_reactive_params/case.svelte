<script>
	import { flip } from 'svelte/animate';
	let items = $state([{ id: 1, name: 'a' }, { id: 2, name: 'b' }]);
	let duration = $state(300);
</script>

{#each items as item (item.id)}
	<div animate:flip={{ duration: duration }}>{item.name}</div>
{/each}
