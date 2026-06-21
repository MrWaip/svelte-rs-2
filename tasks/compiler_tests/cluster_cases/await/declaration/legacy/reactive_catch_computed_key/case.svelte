<svelte:options runes={false} />
<script>
	let p = Promise.reject({ k1: 1 });
	let num = 0;
</script>
{#await p then x}
	ok
{:catch { [`k${num++}`]: v }}
	<button>{v} {num}</button>
{/await}
