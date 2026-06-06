<script>
	const k = "z";
	let v = $state({ z: 1 });
</script>
{#snippet s({ [k]: v })}
	<button>{v}</button>
{/snippet}
{@render s(v)}
