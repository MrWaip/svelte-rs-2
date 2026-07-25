<script>
	let kind = $state('html');
</script>

<button onclick={() => (kind = 'xml')}>toggle</button>

<!doctype html={kind}>
