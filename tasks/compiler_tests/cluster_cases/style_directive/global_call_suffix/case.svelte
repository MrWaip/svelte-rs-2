<script>
	let x = $state(0),
		w = $state(0);
</script>

<button onclick={() => { x++; w++; }}>go</button>
<div style:left="{Math.min(x + 3, w - 10)}px"></div>
