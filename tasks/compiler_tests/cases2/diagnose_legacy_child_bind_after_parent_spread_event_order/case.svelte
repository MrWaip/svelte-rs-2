<svelte:options runes={false} />
<script>
    export let checked = false;
    function k() {}
</script>

<div {...$$restProps} on:click={k}>
    <input bind:checked type="checkbox" />
</div>
