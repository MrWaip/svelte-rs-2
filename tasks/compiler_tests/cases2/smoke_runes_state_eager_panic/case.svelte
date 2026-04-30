<script>
	let eager = $state.eager(0);
	let eagerObj = $state.eager({ x: 0 });

	function script_ops() {
		eager = 1;
		eager++;
		eagerObj.x = 2;
		eagerObj.x++;
	}
</script>

{eager}
{eagerObj.x}
{eager = 3}
{eager++}
{eagerObj.x = 4}
{eagerObj.x++}

<button onclick={script_ops}>run</button>
