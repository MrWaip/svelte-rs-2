<script>
	import A from "./A.svelte";
</script>

<A>
	<span slot="default" let:foo>{foo}</span>
</A>
