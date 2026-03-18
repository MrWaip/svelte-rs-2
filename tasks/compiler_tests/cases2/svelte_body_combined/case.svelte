<script>
	import tooltip from './tooltip.js';

	function handleClick() {
		console.log('clicked');
	}
</script>

<svelte:body onclick={handleClick} use:tooltip={'hello'} />
