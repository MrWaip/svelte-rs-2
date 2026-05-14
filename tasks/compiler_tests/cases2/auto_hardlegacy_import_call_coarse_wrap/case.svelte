<script>
    import Child from './Child.svelte';
    import { tracker } from './tracker';
</script>

<Child track={tracker.click.upgrade()} />
