<script>
    let title = $state("Page");

    function handleClick() {}
</script>

<svelte:head>
    <title>{title}</title>
</svelte:head>

<svelte:body onclick={handleClick} />

<p>Content: {title}</p>
