<script>
	import { tooltip } from './actions.js';
	let { config, value } = $props();
</script>

<label use:tooltip={config}>
	<input type="checkbox" />
	<span>{value}</span>
</label>
