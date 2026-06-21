<svelte:options runes={false} />
<script>
	let p = Promise.resolve({ k1: 1 });
	let num = 0;
</script>
{#await p then { [`k${num++}`]: v }}
	<button>{v} {num}</button>
{/await}
