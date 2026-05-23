<svelte:options runes={false} />
<script>
    export let name = '';
    export let checked = false;
</script>

<input type="checkbox" bind:checked id={$$restProps.id || name} {...$$restProps} />
