<svelte:boundary>
    {@const foo = 1}
    {#snippet pending()}
        <p>{foo}</p>
    {/snippet}
</svelte:boundary>
