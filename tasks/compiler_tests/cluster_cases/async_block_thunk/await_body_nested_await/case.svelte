<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve({ flag: value, list: [value] });
	}
</script>

<button onclick={() => x++}>inc</button>

{#if await (await delay(x)).flag}<p>truthy</p>{/if}
{#each await (await delay(x)).list as item}<p>{item}</p>{/each}
{#key await (await delay(x)).flag}<p>keyed</p>{/key}
{#await await (await delay(x)).flag then value}<p>{value}</p>{/await}
