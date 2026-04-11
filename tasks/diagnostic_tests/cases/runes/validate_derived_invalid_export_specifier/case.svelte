<script>
const count = $state(0);
const total = $derived(count * 2);
export { total };
</script>
