<script>
    import A from './A.svelte';

    let current = A;
    let cond = false;
</script>

<svelte:component this={current}>
    {#if cond}
        <span>child</span>
    {/if}
</svelte:component>
