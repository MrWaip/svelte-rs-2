<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve([value]);
	}

	function join(...args) {
		return args.length;
	}
</script>

<button onclick={() => x++}>inc</button>

{join(...await delay(x))}
{join([...await delay(x)])}
