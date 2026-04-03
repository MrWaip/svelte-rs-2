<script>
	let url = $state('/api');

	export function outer() {
		async function inner() {
			let { data, meta } = $derived(await fetch(url).then((r) => r.json()));
			return 1;
		}

		return inner;
	}
</script>
