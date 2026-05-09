<script>
    let { limitAmount } = $props();
</script>

<input bind:value={$limitAmount} />
<button onclick={() => $limitAmount = undefined}>reset</button>
