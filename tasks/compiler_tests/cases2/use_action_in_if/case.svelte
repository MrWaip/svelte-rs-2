<script>
	import { tooltip } from './actions.js';
	let show = $state(true);
</script>

{#if show}
	<div use:tooltip>text</div>
{/if}
