<script>
	const k = "z";
	let pairs = $state({ z: 1 });
</script>
{#if pairs}
	{@const { [k]: v } = pairs}
	<button>{v}</button>
{/if}
