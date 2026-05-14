<script>
    import Inner from './Inner.svelte';
    import A from './A.svelte';
    import B from './B.svelte';
    export let flag = false;
</script>

<Inner>
    <svelte:component this={flag ? A : B} slot="icon" />
</Inner>
