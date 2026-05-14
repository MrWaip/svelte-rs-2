<script>
    export let cond = false;
</script>

{#if cond}
    {@const xs = [cond ? 1 : 2]}
    {@const ys = [3]}
    {@const all = [...xs, ...ys]}
    {all.length}
{/if}
