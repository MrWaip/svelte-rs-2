<svelte:options runes={false} />
<script>
	const k = "z";
	let v = { z: 1 };
</script>
{#snippet s({ [k]: v })}
	<button>{v}</button>
{/snippet}
{@render s(v)}
