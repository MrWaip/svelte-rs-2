<svelte:options runes={false} />

<script>
    export let value;

    const onClick = (v) => () => {
        value = v;
    };
</script>

<div
    class="chip"
    class:active={value === 1}
    on:click={onClick(1)}
>
    hi
</div>
