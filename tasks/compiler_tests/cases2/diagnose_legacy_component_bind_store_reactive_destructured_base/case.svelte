<svelte:options runes={false} />
<script>
    export let id;
    $: ({ error } = make(id));
</script>

<Child bind:value={$error} />
