<script lang="ts">
    let count = $state(0 satisfies number);
    function inc() { count += 1; }
</script>
<button onclick={inc}>{count}</button>
