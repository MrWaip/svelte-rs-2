<script>
let { id, ...props } = $props();
const label = $derived(props.label + '!');
const style = $derived.by(() => props.style);
const color = $derived(props.style.color);
</script>
<p>{label}</p>
<span>{props.title}</span>
<div>{props.nested.deep.value}</div>
