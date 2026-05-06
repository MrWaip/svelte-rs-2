<script>
	import { items } from './stores';
</script>
{#each $items as item}
	<button onclick={() => item.value++}>{item.value}</button>
{/each}
