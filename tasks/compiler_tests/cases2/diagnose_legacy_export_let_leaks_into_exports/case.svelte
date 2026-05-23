<svelte:options runes={false} />
<script>
	export let a;
	export let b;
	let local = undefined;
	export const getLocal = () => local;
	local = a + b;
</script>
