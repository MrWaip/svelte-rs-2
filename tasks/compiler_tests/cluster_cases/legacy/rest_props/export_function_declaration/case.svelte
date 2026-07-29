<svelte:options runes={false} />
<script>
	export function formatTitle(prefix) {
		return prefix + "!";
	}

	export class Helper {}
</script>

<div {...$$restProps}>{formatTitle("a")}</div>
