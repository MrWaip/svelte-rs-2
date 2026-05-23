<script>
    export let isButton;
    export let onAction;
    export let item;
</script>

<Child onclick={isButton ? () => onAction(item) : undefined} />
