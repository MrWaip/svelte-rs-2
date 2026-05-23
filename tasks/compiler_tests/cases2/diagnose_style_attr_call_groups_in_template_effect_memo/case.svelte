<script>
    let { id = '' } = $props();
    function getStyle() { return 'color:red;'; }
</script>

<div data-testid={id} style={getStyle()}></div>
