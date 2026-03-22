<script>
	import { flip } from 'svelte/animate';
	let { items = [], ...rest } = $props();
</script>

{#each items as item, idx (item.id)}
	<p {...rest} data-index="item-{idx}" animate:flip>{item.name}</p>
{/each}
