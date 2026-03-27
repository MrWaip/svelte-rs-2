<script>
	import Component from './Component.svelte';
	let items = [1, 2, 3];
	let refs = $state([]);
	let obj = { ref: null };
</script>

{#each items as item, i}
	<Component bind:this={refs[i]} />
{/each}

<Component bind:this={obj.ref} />
