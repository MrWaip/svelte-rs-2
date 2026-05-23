<svelte:options runes={false} />
<script>
    export let name = 'n';
</script>

<div id={$$restProps.id || name}></div>
