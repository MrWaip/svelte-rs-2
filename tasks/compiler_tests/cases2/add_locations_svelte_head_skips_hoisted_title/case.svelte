<script>
    let { title = "x" } = $props();
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="x" content="y">
    <link rel="canonical" href="/">
</svelte:head>
