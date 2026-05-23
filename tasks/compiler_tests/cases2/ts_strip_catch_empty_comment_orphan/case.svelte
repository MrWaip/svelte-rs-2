<svelte:options runes={false} />
<script lang="ts">
    export let n: number;
    function run() {
        try {
            const a = 1;
            const b = 2;
            console.log(a, b);
        } catch {
            //
        }
    }
    if (n) {
        console.log(n);
    }
</script>
<button onclick={run}>{n}</button>
