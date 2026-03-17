<script>
	import { tooltip } from './actions.js';
	let items = $state(['a', 'b', 'c']);
</script>

{#each items as item}
	<div {@attach tooltip}>{item}</div>
{/each}
