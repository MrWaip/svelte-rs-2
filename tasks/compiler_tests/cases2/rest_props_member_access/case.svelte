<script>
let { id, ...props } = $props();
const label = $derived(props.label + '!');
</script>
<p>{label}</p>
