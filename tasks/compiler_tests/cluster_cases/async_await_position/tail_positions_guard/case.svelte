<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}

	function pick(first, second) {
		return first + second;
	}

	function wrap(object) {
		return object.value;
	}
</script>

<button onclick={() => x++}>inc</button>

{pick(1, await delay(x))}
{x > 0 ? await delay(x) : 0}
{`value ${await delay(x)}`}
{(0, await delay(x))}
{wrap({ index: 1, value: await delay(x) })}
