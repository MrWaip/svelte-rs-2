<script>
    import Inner from './Inner.svelte';
    export let a = null;
</script>

<Inner let:value={[a]}>
    {@const x = a ? a({ k: 1 }) : null}
    {x}
</Inner>
