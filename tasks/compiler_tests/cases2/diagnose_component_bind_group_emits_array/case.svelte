<script>
    import Child from './Child.svelte';
    let value = $state('a');
</script>

<Child bind:group={value} />
