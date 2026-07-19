<script>
	import Comp from './Comp.svelte';

	let refs = $state([]);
	let items = $state([{ id: 1 }, { id: 2 }]);
</script>

{#each items as item, i (item.id)}
	<Comp bind:this={refs[i]} />
	<button onclick={() => refs[i].foo()}>x</button>
{/each}
