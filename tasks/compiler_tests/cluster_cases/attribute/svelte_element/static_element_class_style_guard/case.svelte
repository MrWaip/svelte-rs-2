<script>
	let value = $state('red');
	const getClass = () => value === 'blue';
	const getValue = () => value;
</script>

<div class:blue={getClass()} style:color={getValue()}></div>
<button onclick={() => value = 'blue'}>toggle</button>
