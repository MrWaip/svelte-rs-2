<script>
    const { x } = $props();
</script>

<div>
    <Cmp>
        {#if x}
            <span>a</span>
        {/if}
        <p>tail</p>
    </Cmp>
</div>
