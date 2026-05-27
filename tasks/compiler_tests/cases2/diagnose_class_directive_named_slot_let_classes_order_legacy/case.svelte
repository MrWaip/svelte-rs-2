<svelte:options runes={false} />

<script>
    export let active;
</script>

<Outer>
    <div class:active slot="activator">
        hi
    </div>
</Outer>
