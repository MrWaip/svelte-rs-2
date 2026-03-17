<script>
	import { tooltip } from './actions.js';
	let config = $state('hello');
</script>

<div use:tooltip={config}>text</div>
