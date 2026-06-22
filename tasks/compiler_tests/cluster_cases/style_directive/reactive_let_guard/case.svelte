<script>
	let col = "red";
</script>
<button on:click={() => col = "blue"}>go</button>
<div style:color={col} />
