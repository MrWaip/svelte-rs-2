<script>
	import List from './List.svelte';
</script>

<List>
	<p slot="item" let:item={{ text }}>{text}</p>
</List>
