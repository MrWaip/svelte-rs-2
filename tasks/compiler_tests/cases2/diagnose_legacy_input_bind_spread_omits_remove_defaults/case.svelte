<svelte:options runes={false} />
<script>
    export let checked = false;
</script>

<input bind:checked type="checkbox" {...$$restProps}>
