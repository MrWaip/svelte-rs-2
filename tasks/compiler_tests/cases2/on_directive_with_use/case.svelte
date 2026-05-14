<svelte:options runes={false} />

<script>
    export let action;
    export let onClick;
    let el;
</script>

<div use:action on:keydown on:click={onClick} bind:this={el}>
    <slot />
</div>
