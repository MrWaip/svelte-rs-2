<script>
    export let items;
    let inserted = false;

    function shouldShow() {
        if (inserted) {
            return false;
        }
        inserted = true;
        return true;
    }
</script>

{#each items as item}
    {#if shouldShow()}<p>{item}</p>{/if}
{/each}
