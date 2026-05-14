<script>
    let classes = $state([]);
    function mapClasses(base, ...rest) {
        return { [base]: true };
    }
</script>

<div {...mapClasses('base', ...classes)} class:active={true} />
