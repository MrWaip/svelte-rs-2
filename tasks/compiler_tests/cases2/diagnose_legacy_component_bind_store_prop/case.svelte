<svelte:options runes={false} />
<script>
    export let error;
</script>

<Child bind:value={$error} />
