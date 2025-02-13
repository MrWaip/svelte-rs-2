<script>
	import {onMount} from 'svelte';
	let state = $state("");
	let counter = $state(0);

	counter = 10;
</script>

<div>
	Lorem {state} + {state} = Ipsum;
	<div
			class:state
			class:staticly={true}
			class:invinsible
			class:reactive={counter}
		>
			sequence	

			{#if state}
					<span title="{state}__________{state}" empty {state} counter={counter}>
						text	
					</span>
			{:else}
				<div>
					<input {title} {state} />
				</div>

				{#if counter > 30}
					<h1 {state}>
						 	text	
					</h1>
					{:else if  counter == 100}
						 text	
				{:else}
					<h2>
						EMPTY
					</h2>
				{/if}
				
			{/if}
		
	</div>
</div>
