<script>
    import Inner from './Inner.svelte';
    let current = Inner;
</script>

<svelte:component this={current}>
    <span slot="caption">{'hi'}</span>
</svelte:component>
