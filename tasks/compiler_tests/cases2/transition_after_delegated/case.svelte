<script>
	import { slide } from 'svelte/transition';
	let visible = $state(true);
	function k() {}
	function c() {}
</script>

{#if visible}
	<div transition:slide onkeydown={k} onclick={c}>hi</div>
{/if}
