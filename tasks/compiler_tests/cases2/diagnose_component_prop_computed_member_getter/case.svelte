<script>
    const obj = $derived({ 'data-testid': 'x' });
</script>
<Comp dataTestid={obj['data-testid']} />
