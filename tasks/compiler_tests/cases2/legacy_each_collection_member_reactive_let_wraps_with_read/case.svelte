<svelte:options runes={false} />
<script>
    const filters = [
        { data: [1] },
        { data: [2] },
    ];

    let modeData = filters[0];
</script>

<button onclick={() => { modeData = filters[1]; }}>swap</button>

{#each modeData.data as curtain (curtain)}
    <div>{curtain}</div>
{/each}
