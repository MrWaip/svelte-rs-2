<script>
	import Parent from "./Parent.svelte";
	import Child from "./Child.svelte";
</script>

<Parent>
	<Child slot="item" let:foo={{ a, b }}>{a}{b}</Child>
</Parent>
