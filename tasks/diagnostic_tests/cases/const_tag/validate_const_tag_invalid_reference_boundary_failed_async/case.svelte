<svelte:boundary>
    {@const foo = 1}
    {#snippet failed(error, reset)}
        <p>{foo}</p>
    {/snippet}
</svelte:boundary>
