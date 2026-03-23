<script>
let a = $state(1);
let b = $state(2);

export function compute() {
    const sum = $derived(a + b);
    return sum;
}
</script>
