<script>
	import Parent from "./Parent.svelte";
	export let component;
</script>

<Parent>
	<svelte:component this={component} slot="item" let:item let:index {item} {index} />
</Parent>
