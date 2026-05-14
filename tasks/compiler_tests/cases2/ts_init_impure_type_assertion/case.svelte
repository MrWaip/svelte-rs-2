<script lang="ts">
    let count = 0;
    function tick(): number { count += 1; return count; }
    let show = true;
</script>
{#if show}
    {@const value = <number>tick()}
    <p>{value}</p>
{/if}
