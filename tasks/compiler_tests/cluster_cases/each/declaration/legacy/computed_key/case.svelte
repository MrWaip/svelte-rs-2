<svelte:options runes={false} />
<script>
	const k = "z";
	let items = [{ z: 1 }];
</script>
{#each items as { [k]: v }}
	<button>{v}</button>
{/each}
