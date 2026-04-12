<script>
    let open = true;
</script>

<style>
    [data-state="open"] { color: red; }
</style>

<div data-state={open ? "open" : "closed"}>inside</div>
<div data-state="closed">outside</div>
