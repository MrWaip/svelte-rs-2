<script>
    let { children } = $props();
</script>

<A />
{@render children?.()}

<svelte:head>
    <style>html { height: 100%; }</style>
</svelte:head>
