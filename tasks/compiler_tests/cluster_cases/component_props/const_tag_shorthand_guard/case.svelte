<script>
	import Child from './Child.svelte';
	let live = $state(0);
	let plain = 7;
	function bump() {
		live++;
	}
</script>

{#snippet row()}
	{@const kLit = 'x'}
	{@const kLive = live + 1}
	{@const kCall = Math.random()}
	<Child {kLive} {kCall} {plain} eLit={kLit} eLive={kLive} />
{/snippet}

{@render row()}
<button onclick={bump}>b</button>
