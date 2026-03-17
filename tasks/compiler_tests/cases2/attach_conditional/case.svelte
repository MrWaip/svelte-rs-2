<script>
	import { tooltip } from './actions.js';
	let enabled = $state(true);
</script>

<div {@attach enabled && tooltip}>hello</div>
