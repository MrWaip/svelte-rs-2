<script>
	import Inner from './Inner.svelte';

	let message = $state('one');

	function attachment(message) {
		return (node) => {
			node.textContent = message;
		};
	}
</script>

<Inner {@attach attachment(message)} />
