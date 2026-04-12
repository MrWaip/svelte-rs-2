<script>
	import List from './List.svelte';
</script>

<List let:item={processed}>
	<p>{processed.text}</p>
</List>
