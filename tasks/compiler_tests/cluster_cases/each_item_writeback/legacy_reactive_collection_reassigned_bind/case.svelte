<svelte:options runes={false} />
<script>
	let todos = [{ done: false }];
	function update() { todos = todos.slice(1); }
</script>

<button on:click={update}>x</button>
{#each todos as todo}
	<input type="checkbox" bind:checked={todo.done} />
{/each}
