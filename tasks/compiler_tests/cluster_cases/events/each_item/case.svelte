<script>
	let items = $state([() => {}]);
</script>

{#each items as handler}
	<button onclick={handler}>x</button>
{/each}
