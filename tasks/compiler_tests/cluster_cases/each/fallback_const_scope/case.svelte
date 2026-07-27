{#each { length: 1 }}
	{@const data = 1}
	{data}
{/each}

{#each { length: 0 }}
	x
{:else}
	{@const data = 2}
	{data}
{/each}
