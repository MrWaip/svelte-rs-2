<script>
	let x = $state('hi');
</script>

<select><option><b>{x}</b></option></select>
<select><option><b>{x}</b></option></select>

<button onclick={() => (x = 'bye')}>x</button>
