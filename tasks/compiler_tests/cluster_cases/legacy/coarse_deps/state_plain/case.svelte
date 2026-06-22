<script>
    let count = 1;

    function inc() {
        count = count + 1;
    }
</script>

<button on:click={inc}>+</button>

{#if count}
    {@const label = count.toFixed(2)}
    <span>{label}</span>
{/if}
