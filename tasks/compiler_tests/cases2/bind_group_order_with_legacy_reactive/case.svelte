<script>
    export let value;
    export let group;
    $: id = `${value}-radio`;
</script>

<input type="radio" bind:group={group} {id} {value} />
