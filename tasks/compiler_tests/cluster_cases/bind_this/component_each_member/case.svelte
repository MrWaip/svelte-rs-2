<script>
	import Comp from './Comp.svelte';
	export let items = [{ ref: null }];
</script>

{#each items as item}
	<Comp bind:this={item.ref}/>
{/each}
