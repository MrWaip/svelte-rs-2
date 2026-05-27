<script>
    let { cond = false } = $props();
</script>

<div class="header" class:slot={cond}>hi</div>
