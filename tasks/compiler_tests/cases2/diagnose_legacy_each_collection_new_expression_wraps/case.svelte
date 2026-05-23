<svelte:options runes={false} />

{#each new Array(4).fill(null) as _, i}
    <span>{i}</span>
{/each}
