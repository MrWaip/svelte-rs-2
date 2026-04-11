<script>
    let html = $state('');
    let editable = true;
</script>
<div contenteditable={editable} bind:innerHTML={html}></div>
