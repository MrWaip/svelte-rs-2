<script>
    let { inputValue = $bindable() } = $props();
    function set5() { inputValue.set(5); }
</script>
<button onclick={set5}>read: {$inputValue}</button>
<input bind:value={$inputValue} />
