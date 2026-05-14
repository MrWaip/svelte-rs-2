<svelte:options runes={false} />

<script>
	export let onInput = () => {};

	let flag = false;

	$: handler = async (value) => {
		const result = await onInput(value, flag);
		if (result) {
			flag = true;
		}
	};
</script>

<button on:click={handler}>go</button>
