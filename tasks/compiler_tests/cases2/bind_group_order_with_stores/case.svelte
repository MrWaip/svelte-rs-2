<script>
    import { writable } from "svelte/store";
    let metrics = writable([1, 2, 3]);
    let group = $state([]);
    let total = $derived($metrics.length);
</script>

<input type="radio" bind:group={group} value="a" />
<input type="radio" bind:group={group} value="b" />
<p>{total}</p>
