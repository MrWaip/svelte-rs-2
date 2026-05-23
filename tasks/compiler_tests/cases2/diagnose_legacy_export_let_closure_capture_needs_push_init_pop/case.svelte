<svelte:options runes={false} />
<script>
    export let duration;
    const opts = () => ({ duration });
</script>

<p>{opts().duration}</p>
