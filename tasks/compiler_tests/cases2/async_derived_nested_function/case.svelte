<script>
	let url = $state('/api');

	export function outer() {
		async function inner() {
			let data = $derived(await fetch(url));
			return data;
		}

		return inner;
	}
</script>
