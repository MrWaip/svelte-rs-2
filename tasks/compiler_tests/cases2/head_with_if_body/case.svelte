<script>
    let cond = $state(false);
</script>

<svelte:head>
    <title>t</title>
</svelte:head>

{#if cond}
    <span>a</span>
{:else}
    <span>b</span>
{/if}
