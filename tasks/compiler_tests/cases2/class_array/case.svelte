<script>
    let active = $state(false);
    const base = "btn";
</script>

<div class={[base, active && "active", "extra"]}>content</div>
