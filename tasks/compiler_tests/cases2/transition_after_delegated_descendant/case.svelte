<script>
	import { slide } from 'svelte/transition';
	let visible = $state(true);
	function k() {}
</script>

{#if visible}
	<div transition:slide>
		<button onclick={k}>hi</button>
	</div>
{/if}
