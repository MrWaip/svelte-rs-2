<script>
	import { focus, tooltip } from './actions.js';
	let config = { text: 'hello' };
</script>

<div use:focus use:tooltip={config}>text</div>
