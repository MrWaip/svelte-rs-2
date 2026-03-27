<script>
    let { tag = "div" } = $props();
    let title = $state("hello");
</script>

<svelte:element this={tag} class="first">
    First: {title}
</svelte:element>

<svelte:element this={tag} class="second">
    Second: {title}
</svelte:element>