<svelte:options customElement="my-el" />

<script>
	let { children } = $props();
</script>

<slot />
{@render children?.()}
