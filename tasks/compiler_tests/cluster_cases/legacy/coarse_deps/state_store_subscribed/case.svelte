<script>
    let state = "hello";

    function update() {
        state = state + "!";
    }
</script>

<button on:click={update}>update</button>

{#if state}
    {@const len = state.length}
    <span>{len} / {$state}</span>
{/if}
