<script>
    import Child from './Child.svelte';
    let inputRef = $state();
</script>

<Child bind:ref={inputRef} />
