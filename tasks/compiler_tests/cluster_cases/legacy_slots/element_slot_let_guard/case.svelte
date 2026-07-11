<script>
	import Parent from "./Parent.svelte";
</script>

<Parent>
	<div slot="item" let:item>{item}</div>
</Parent>
