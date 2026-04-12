<script>
	import List from './List.svelte';
</script>

<List>
	<svelte:fragment slot="item" let:item>
		<p>{item.text}</p>
	</svelte:fragment>
</List>
