<script>
	import { animations } from './utils';
	let items = $state([{ id: 1, name: 'a' }, { id: 2, name: 'b' }]);
</script>

{#each items as item (item.id)}
	<div animate:animations.flip>{item.name}</div>
{/each}
