<svelte:options customElement="my-el" />

<script>
	let { children } = $props();
</script>

{$$slots}
<slot />
{@render children?.()}
