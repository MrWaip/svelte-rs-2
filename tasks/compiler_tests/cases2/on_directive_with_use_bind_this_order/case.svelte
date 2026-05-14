<svelte:options runes={false} />

<script>
    export let action;
    export let onClick;
    export let onKey;
    let el;
</script>

<div on:keydown={onKey} use:action bind:this={el} on:click={onClick}></div>
