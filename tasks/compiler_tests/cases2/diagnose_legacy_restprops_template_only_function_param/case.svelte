<script>
	import Child from './Child.svelte';

	function wrap(props) {
		return props;
	}
</script>

<Child {...wrap($$restProps)} />
