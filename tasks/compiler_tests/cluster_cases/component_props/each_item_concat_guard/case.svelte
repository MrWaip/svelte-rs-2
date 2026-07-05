<script>
	let items = $state([{ text: 't' }]);
</script>

{#each items as badge, i}
	<Badge dataTestid="badge-{badge.text}" />
{/each}
