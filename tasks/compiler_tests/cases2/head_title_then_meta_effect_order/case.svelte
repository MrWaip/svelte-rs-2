<script>
    let { title } = $props();
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="og:title" content={title} />
</svelte:head>
