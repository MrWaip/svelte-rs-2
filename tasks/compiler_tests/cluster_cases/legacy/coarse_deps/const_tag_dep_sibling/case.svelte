<script>
    export let obj;
</script>

{#if obj}
    {@const name = obj.name}
    {@const len = name.length}
    <span>{name}: {len}</span>
{/if}
