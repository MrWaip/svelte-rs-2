<script>
	import Parent from "./Parent.svelte";
	import Child from "./Child.svelte";
</script>

<Parent>
	<Child slot="item" let:item let:index {item} {index} />
</Parent>
