<script>
    let dynamicEl;
    let counter = $state(0);
</script>

<div
    class:state={counter > 0}
    bind:this={dynamicEl}
>
    {#if counter}
        <span>x</span>
    {/if}
</div>
