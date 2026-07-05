<script>
	let items = $state([{ id: 1 }]);
</script>

{#each items as badge, i (badge.id)}
	<Badge dataTestid="badge-{i}" />
{/each}
