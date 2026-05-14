<script>
    import Child from './Child.svelte';
    let a = $state('x');
    let b = $state('y');
</script>

<Child bind:group={a} />
<Child bind:group={b} />
