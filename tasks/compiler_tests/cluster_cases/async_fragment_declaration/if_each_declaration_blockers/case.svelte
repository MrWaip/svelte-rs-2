<svelte:boundary>
	{@const number = await Promise.resolve(5)}
	{#if number > 4}
		{@const length = await number}
		{#each { length }, index}
			{index}
		{/each}
	{/if}
</svelte:boundary>
