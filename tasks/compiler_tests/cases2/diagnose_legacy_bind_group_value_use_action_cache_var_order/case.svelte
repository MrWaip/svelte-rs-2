<svelte:options runes={false} />
<script>
	export let model = "a";
	export let value = "a";
	function action() {}
</script>

<input
	type="radio"
	{value}
	bind:group={model}
	use:action
/>
