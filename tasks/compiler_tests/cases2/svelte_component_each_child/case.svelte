<script>
    import A from './A.svelte';

    let current = A;
    let items = [1, 2, 3];
</script>

<svelte:component this={current}>
    {#each items as item}
        <span>{item}</span>
    {/each}
</svelte:component>
