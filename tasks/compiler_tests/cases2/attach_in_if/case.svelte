<script>
	import { tooltip } from './actions.js';
	let show = $state(true);
</script>

{#if show}
	<div {@attach tooltip}>hello</div>
{/if}
