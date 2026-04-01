<script>
	let props = $state({ id: "a", style: "border-color: blue;" });
	let color = $state("red");
</script>

<div {...props} style:color={color} />
