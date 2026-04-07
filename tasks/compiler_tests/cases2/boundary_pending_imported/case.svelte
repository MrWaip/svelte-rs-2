<script>
	import { pending } from './handlers.js';
</script>

<svelte:boundary pending={pending}>
	<p>content</p>
</svelte:boundary>
