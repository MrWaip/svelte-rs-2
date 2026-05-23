<svelte:options runes={false} />

<script>
    export let flag = false;
    export let Comp;
    function onA() {}
    function onB() {}
</script>

<svelte:component
    this={Comp}
    onA={flag ? onA : undefined}
    onB={flag ? onB : undefined}
/>
