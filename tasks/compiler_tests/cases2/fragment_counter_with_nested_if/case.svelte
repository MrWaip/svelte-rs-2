<script>
    let count = $state(0);
    let visible = $state(true);
</script>

<div>
    {#if visible}
        <span>{count}</span>
    {:else}
        <div>
            <input value={count} />
        </div>

        {#if count > 10}
            <h1>Big</h1>
        {:else}
            <h2>Small</h2>
        {/if}
    {/if}
</div>

<div>
    {#if visible}
        <span>{count}</span>
    {:else}
        <div>
            <input value={count} />
        </div>

        {#if count > 10}
            <h1>Big</h1>
        {:else}
            <h2>Small</h2>
        {/if}
    {/if}
</div>
