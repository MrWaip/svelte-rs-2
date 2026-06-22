<svelte:options runes={false} />
<script>
	let p = Promise.resolve({ k0: 1 });
	let num = 0;
</script>
<button on:click={() => num++}>inc</button>
{#await p then { [`k${num}`]: v }}
	<button>{v}</button>
{/await}
