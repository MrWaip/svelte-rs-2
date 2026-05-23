<script lang="ts">
    class Counter {
        x: number;
        constructor() {
            this.x = $state(0) as number;
        }
    }
    const c = new Counter();
</script>
<p>{c.x}</p>
