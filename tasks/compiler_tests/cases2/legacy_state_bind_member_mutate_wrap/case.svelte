<svelte:options runes={false} />
<script>
    let obj = { x: 1 };
</script>

<input bind:value={obj.x} />
