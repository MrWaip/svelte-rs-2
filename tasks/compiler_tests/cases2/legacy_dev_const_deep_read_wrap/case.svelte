<script>
    let state = '';

    function bump() {
        state = state + 'x';
    }
</script>

<button onclick={bump}>bump</button>

{#if state}
    {@const localLen = state.length}
    <span>Length: {localLen}</span>
{/if}
