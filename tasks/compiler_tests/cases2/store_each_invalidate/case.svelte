<script>
	import { items } from './stores';
</script>

{#each $items as item}
	<p>{item}</p>
{/each}
