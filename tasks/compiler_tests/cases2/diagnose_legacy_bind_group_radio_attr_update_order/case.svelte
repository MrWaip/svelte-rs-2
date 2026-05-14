<svelte:options runes={false} />
<script>
	export let model = "a";
	export let value = "a";
	export let name = "radio";
	export let hasError = false;
</script>

<input
	type="radio"
	{value}
	{name}
	class:error={hasError}
	on:click
	bind:group={model}
/>
