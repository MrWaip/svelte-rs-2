<script>
	import { tooltip } from './actions.js';
	let items = $state(['a', 'b', 'c']);
</script>

{#each items as item}
	<div use:tooltip={item}>text</div>
{/each}
