<script>
	import Parent from "./Parent.svelte";
</script>

<Parent>
	<svelte:fragment slot="item" let:item>{item}</svelte:fragment>
</Parent>
