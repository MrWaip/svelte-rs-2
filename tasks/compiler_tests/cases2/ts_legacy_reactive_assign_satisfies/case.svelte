<script lang="ts">
    let count = 0;
    $: (doubled = count * 2) satisfies number;
</script>
<p>{doubled}</p>
