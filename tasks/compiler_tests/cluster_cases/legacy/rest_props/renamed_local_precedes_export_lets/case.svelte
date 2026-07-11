<svelte:options runes={false} />
<script>
	let className = '';
	export let data;
	export let scale = 1;
	export { className as class };
</script>

<div {...$$restProps} class={className}>{data}{scale}</div>
