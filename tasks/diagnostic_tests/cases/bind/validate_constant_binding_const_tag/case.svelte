<script>
	let array = [1, 2, 3];
</script>

{#each array as a}
	{@const b = a + 1}
	<input bind:value={b} />
{/each}
