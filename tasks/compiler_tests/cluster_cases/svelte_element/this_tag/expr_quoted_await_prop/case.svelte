<script>
	let { pending } = $props();
</script>
<svelte:element this="{await pending}">hello</svelte:element>
