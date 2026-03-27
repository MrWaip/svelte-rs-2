<script>
	let count = $state(0);
	let data = $state(null);

	const handleArrow = () => {
		$inspect.trace();
		count++;
	};

	async function fetchData() {
		$inspect.trace();
		data = await fetch('/api');
	}

	foo(() => {
		$inspect.trace();
		count++;
	});

	const obj = {
		handler() {
			$inspect.trace();
			count++;
		}
	};
</script>

<button onclick={() => {
	$inspect.trace();
	count++;
}}>Click</button>
