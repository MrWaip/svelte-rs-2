<script>
	import List from './List.svelte';
</script>

<List>
	<svelte:fragment slot="item" let:item={{ text }}>
		<p>{text}</p>
	</svelte:fragment>
</List>
