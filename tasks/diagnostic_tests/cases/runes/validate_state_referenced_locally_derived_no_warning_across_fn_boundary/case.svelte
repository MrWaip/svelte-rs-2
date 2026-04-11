<script>
let count = $state(0);
let total = $derived(count * 2);
let x = $state(() => total);
</script>
