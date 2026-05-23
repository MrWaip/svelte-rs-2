<script>
    export let onClick;
</script>

<svelte:body on:click|preventDefault={onClick} />
