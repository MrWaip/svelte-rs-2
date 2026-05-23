<script lang="ts">
    class Counter {
        x = ($state(0));
    }
    const c = new Counter();
</script>
<p>{c.x}</p>
