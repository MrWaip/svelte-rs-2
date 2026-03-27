<script>
    let { items = [] } = $props();
    let state = $state("test");
</script>

<div>
    {#if state}
        <span>{state}</span>
    {:else}
        <div><input value={state} /></div>
    {/if}
</div>

<div>
    {#if state}
        <span>{state}</span>
    {:else}
        <div><input value={state} /></div>
    {/if}
</div>