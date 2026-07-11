<script>
	import Parent from "./Parent.svelte";
	import Child from "./Child.svelte";
</script>

<Parent>
	<Child slot="item" let:item onclick={() => item}>asd</Child>
</Parent>
