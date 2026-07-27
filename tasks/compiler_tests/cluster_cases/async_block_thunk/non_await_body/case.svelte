<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}

	function delay_list(value) {
		return Promise.resolve([value]);
	}
</script>

<button onclick={() => x++}>inc</button>

{#if await delay(x) > 0}<p>positive</p>{/if}
{#each await delay_list(x) || [] as item}<p>{item}</p>{/each}
{#key await delay(x) + 1}<p>keyed</p>{/key}
{#await await delay(x) + 1 then value}<p>{value}</p>{/await}
