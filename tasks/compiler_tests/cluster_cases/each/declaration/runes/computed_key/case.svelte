<script>
	const k = "z";
	let items = $state([{ z: 1 }]);
</script>
{#each items as { [k]: v }}
	<button>{v}</button>
{/each}
