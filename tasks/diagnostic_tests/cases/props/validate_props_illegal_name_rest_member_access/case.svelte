<script>
let { x, ...rest } = $props();
console.log(rest.$$slots);
</script>
