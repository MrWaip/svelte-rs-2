<script>
    let id = $state();
</script>
{#if true}
    <div name="123">
        {id}

        {#if id}
            120 = {id}
        {/if}
    </div>
{/if}