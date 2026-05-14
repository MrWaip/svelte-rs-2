<script>
    import Icon from './Icon.svelte';
    let { color } = $props();
    let current = Icon;
</script>

<span class="wrap">
    <svelte:component this={current} --my-color="var(--{color})" />
</span>

<style>
    .wrap { display: flex; }
</style>
