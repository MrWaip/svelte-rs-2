<script>
	import List from './List.svelte';
</script>

<List>
	<svelte:fragment slot="row" let:row>
		{@const v = row.value * 2}
		<p>{v}</p>
	</svelte:fragment>
</List>
