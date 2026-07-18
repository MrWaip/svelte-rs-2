<script>
	let { cls } = $props();
</script>
{const active = cls === "on"}
<div class={active ? "a" : "b"}>{active}</div>
