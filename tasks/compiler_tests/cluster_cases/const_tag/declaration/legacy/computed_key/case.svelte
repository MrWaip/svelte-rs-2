<svelte:options runes={false} />
<script>
	const k = "z";
	let pairs = { z: 1 };
</script>
{#if pairs}
	{@const { [k]: v } = pairs}
	<button>{v}</button>
{/if}
