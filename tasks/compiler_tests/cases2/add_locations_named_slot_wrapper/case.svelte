<script>
    import Widget from './Widget.svelte';
    let { value = "x" } = $props();
</script>

<Widget>
    <div slot="footer">Footer: {value}</div>
</Widget>
