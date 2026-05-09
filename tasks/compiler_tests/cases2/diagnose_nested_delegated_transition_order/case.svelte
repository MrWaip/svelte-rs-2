<script>
    import { fade, fly } from 'svelte/transition';
    const handler = () => {};
</script>

<div out:fade onclick={handler}>
    <div in:fly onclick={handler}></div>
</div>
