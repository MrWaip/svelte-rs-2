<script lang="ts">
    let count = $state(0);
    let { val = (count) } = $props();
</script>
<p>{val}</p>
