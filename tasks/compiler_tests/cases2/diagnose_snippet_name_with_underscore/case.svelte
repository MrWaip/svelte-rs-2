<script>
    import Child from './Child.svelte';
</script>

<Child>
    {#snippet extra_element()}
        <span>hi</span>
    {/snippet}
</Child>
