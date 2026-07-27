<script>
	let x = $state('hi');
</script>

<select><option><span>static</span></option></select>
<select><option><span>{x}</span></option></select>

<button onclick={() => (x = 'bye')}>x</button>
