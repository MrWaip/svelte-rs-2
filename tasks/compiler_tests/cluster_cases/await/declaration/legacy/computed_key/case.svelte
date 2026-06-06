<svelte:options runes={false} />
<script>
	const k = "z";
	let p = Promise.resolve({ z: 1 });
</script>
{#await p then { [k]: v }}
	<button>{v}</button>
{/await}
