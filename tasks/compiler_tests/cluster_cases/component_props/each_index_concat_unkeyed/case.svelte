<script>
	let items = $state([{ text: 't' }]);
</script>

{#each items as badge, i}
	<Badge text={badge.text} dataTestid="badge-{i}" />
{/each}
