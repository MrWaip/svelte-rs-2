<script>
    import Outer from './Outer.svelte';
    import Inner from './Inner.svelte';
    import Leaf from './Leaf.svelte';
</script>

<Outer>
    <Inner slot="action">
        <svelte:fragment let:y>
            <Leaf value={y} />
        </svelte:fragment>
    </Inner>
</Outer>
