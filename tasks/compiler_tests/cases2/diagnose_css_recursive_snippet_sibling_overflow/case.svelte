{#snippet a()}
    {@render b()}
    <div>
        {@render b()}
    </div>
{/snippet}

{#snippet b()}
    {@render a()}
    <div>
        {@render a()}
    </div>
{/snippet}

<style>
    div + div {
        color: green;
    }
</style>
