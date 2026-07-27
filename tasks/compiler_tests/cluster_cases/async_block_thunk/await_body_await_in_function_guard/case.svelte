<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}

	function call(callback) {
		return callback();
	}
</script>

<button onclick={() => x++}>inc</button>

{#if await call(async () => await delay(x))}<p>truthy</p>{/if}
{#key await call(async () => await delay(x))}<p>keyed</p>{/key}
