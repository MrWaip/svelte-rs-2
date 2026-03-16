<script>
    let isActive = $state(false);
    const bold = true;
</script>

<div class={{ active: isActive, bold }}>content</div>
