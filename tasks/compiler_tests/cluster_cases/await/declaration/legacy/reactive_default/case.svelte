<svelte:options runes={false} />
<script>
	let p = Promise.resolve({});
	let num = 0;
</script>
<button on:click={() => num++}>inc</button>
{#await p then { v = num }}
	<button>{v}</button>
{/await}
