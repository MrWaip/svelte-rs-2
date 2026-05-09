<script>
    let x = $state(0);
</script>

{#snippet inner(mf)}
    <div>
        <div>
            <div>
                <div><span>a</span></div>
            </div>
        </div>
        <div>
            {#key x}
                {@render mf()}
            {/key}
        </div>
    </div>
{/snippet}
