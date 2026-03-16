<script>
    let active = $state(false);
    let highlighted = $state(true);
    const base = "btn";
</script>

<div class={[base, active && "active"]} class:highlighted>content</div>
