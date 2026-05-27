<svelte:options runes={false} />

<script>
    export let onclick = undefined;
    export let useFn = undefined;
    export let useArgs = [];
    export let href = undefined;

    function getTag() {
        return href ? 'a' : 'div';
    }
</script>

<svelte:element
    this={getTag()}
    use:useFn={useArgs || []}
    {href}
    on:click={onclick}
>x</svelte:element>
