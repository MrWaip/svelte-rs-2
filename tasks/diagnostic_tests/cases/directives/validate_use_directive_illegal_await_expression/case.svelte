<script>
	import { tooltip } from './actions.js';

	async function load() {
		return { text: 'hello' };
	}
</script>

<div use:tooltip={await load()}></div>
