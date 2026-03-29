{#await items then result}
	<ul>
		{#each result as item}
			<li>{item}</li>
		{/each}
	</ul>
{/await}