<script>
	import { tooltip } from './actions.js';
	import { fade } from 'svelte/transition';
	let value = $state('');
</script>

<input {@attach tooltip} bind:value transition:fade />
