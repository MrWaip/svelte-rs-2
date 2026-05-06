<script>
	import { items } from './stores';
</script>

{#each $items as item}
	<p>{item.name}</p>
	<button onclick={() => item.name = 'x'}>edit</button>
{/each}
