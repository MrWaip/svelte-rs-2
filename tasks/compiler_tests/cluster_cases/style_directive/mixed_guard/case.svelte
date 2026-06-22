<script>
	let a = "red";
	let b = "blue";
</script>
<button on:click={() => b = "green"}>go</button>
<div style:color={a} style:background={b} />
