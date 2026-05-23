<svelte:options runes={false} />
<script>
    export let value = undefined;
</script>

<input type="radio" bind:group={value} value="a" />
