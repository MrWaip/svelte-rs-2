<script>
	import { handler } from './handlers.js';
</script>

<svelte:boundary onerror={handler}>
	<p>content</p>
</svelte:boundary>
