<script>
	let count = $state(0);
	let double = $derived(count * 2);
	export { double };
</script>

<button onclick={() => count++}>{double}</button>
