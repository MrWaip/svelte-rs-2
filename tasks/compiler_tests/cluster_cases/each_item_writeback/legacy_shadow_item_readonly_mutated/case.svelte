<svelte:options runes={false} />
<script>
	let a = ['Hello'];
	function go() {
		a = [...a, 'x'];
	}
</script>

{#each a as a}
	{a}
{/each}
<button on:click={go}>go</button>
