<script>
	import List from './List.svelte';
</script>

<List>
	<p slot="item" let:item={{ text }} let:index>{text} {index}</p>
</List>
