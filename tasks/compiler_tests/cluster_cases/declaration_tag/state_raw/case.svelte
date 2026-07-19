{#each items as item}
	{let box = $state.raw({ w: item })}
	<p>{box.w}</p>
{/each}
