<svelte:options runes={false} />

<script>
    let component;
</script>

<svelte:component this={component}>
    <svelte:fragment slot="empty-state">
        empty
    </svelte:fragment>
</svelte:component>
