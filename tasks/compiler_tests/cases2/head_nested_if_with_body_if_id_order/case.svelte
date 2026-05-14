<script>
    let { cond = true, show = true } = $props();
</script>

{#if cond}
    <button>x</button>
{/if}
<svelte:head>
    {#if show}
        <meta name="a" content="b" />
    {/if}
</svelte:head>
