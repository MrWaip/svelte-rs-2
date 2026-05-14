<script lang="ts">
    async function fetchValue(): Promise<number> { return 5; }
    let x = $derived((await fetchValue()));
</script>
<p>{x}</p>
