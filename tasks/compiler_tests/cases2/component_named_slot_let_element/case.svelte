<script>
	import List from './List.svelte';
</script>

<List>
	<p slot="item" let:item>{item.text}</p>
</List>
