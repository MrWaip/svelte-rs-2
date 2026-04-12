<script>
	let { user = { profile: {} } } = $props();
	let key = 'name';

	function rename() {
		user.profile[key] = 'next';
	}
</script>
