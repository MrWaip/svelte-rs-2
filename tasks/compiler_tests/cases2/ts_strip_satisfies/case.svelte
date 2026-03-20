<script lang="ts">
    type Count = number;
    let count: Count = $state(0);
    count++;
</script>

<p>{count satisfies Count}</p>
