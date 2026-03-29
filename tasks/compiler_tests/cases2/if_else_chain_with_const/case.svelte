<script>
    let count = $state(0);
    let name = $state("world");
</script>

{#if count > 100}
    {@const label = name + "!"}
    <h1>{label}</h1>
{:else if count > 50}
    <h2>Medium: {count}</h2>
{:else if count > 10}
    {@const small = count * 2}
    <h3>Small doubled: {small}</h3>
{:else}
    <p>Tiny: {count}</p>
{/if}
