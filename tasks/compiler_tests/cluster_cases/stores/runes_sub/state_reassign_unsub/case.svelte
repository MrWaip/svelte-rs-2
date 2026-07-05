<script>
	import { writable } from "svelte/store";
	let watcherA = $state();
</script>

{#if watcherA}
	{$watcherA}
	<button onclick={() => watcherA = null}>remove</button>
{:else}
	<button onclick={() => watcherA = writable(0)}>add</button>
{/if}
