<svelte:options css="injected" />

<script>
    import { writable } from "svelte/store";
    let store = writable(0);
    let count = $state(0);
</script>

<style>
    p { color: red; }
</style>

<p>{$store} {count}</p>
