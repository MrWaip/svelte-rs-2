<script>
	import List from './List.svelte';
</script>

<List>
	<svelte:fragment let:item>
		<p>{item.text}</p>
	</svelte:fragment>
</List>
