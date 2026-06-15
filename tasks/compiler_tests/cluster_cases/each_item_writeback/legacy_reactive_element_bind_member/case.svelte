<svelte:options runes={false} />
<script>
	let todos = [{ done: false }];
</script>

{#each todos as todo}
	<input type="checkbox" bind:checked={todo.done} />
{/each}
