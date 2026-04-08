<script module>
	export const KIND = "v1";
	export function label(name) {
		return `${KIND}:${name}`;
	}
</script>

<script>
	let { title } = $props();
</script>

{#snippet row(text)}
	<span>{text}</span>
{/snippet}

{@render row(label(title))}
