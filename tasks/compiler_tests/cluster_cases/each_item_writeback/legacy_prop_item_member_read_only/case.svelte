<svelte:options runes={false} />
<script>
	export let arr = [{ prop: "foo" }];
</script>

{#each arr as o}
	<span>{o.prop}</span>
{/each}
