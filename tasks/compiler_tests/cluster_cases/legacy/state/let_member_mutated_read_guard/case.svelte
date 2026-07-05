<script>
	let obj = { k: 1 };
	const bump = () => {
		obj.k = 2;
	};
</script>

<button onclick={bump}>{obj.k}</button>
