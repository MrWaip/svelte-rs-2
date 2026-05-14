<script>
    import Child from './Child.svelte';

    let count = $state(0);
</script>

<button onclick={() => count++}>{count}</button>
<Child aria-disabled={!count}>hi</Child>
