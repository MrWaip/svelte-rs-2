<svelte:options runes={false} />
<script lang="ts">
	type SomeType = { id: number };

	export let name: string | undefined;
	export let typed: SomeType | null | (() => void) = null;
</script>

<p>{name}</p>
<p>{typed}</p>
