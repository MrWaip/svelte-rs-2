<script>
let x = $state(0);

export function getValues() {
    const doubled = $derived(x * 2);
    return {
        doubled,
        get live() {
            return doubled;
        }
    };
}
</script>
