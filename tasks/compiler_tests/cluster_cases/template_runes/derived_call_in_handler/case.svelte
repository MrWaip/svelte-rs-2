<script>
	const noop = () => {};
</script>

<button onclick={() => { let count = $state(0); let double = $derived(count * 2); noop(double); }}>go</button>
