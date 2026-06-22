<svelte:options runes={false} />
<script>
	let todos = [{ id: 1, done: false }];
</script>

{#each todos as todo (todo.id)}
	<input type="checkbox" bind:checked={todo.done} />
{/each}
