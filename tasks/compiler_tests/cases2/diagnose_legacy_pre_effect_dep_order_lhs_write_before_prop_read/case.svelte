<script>
	export let prop;
	let local = 0;
	let out = 0;

	$: if (true) {
		local = 1;
		out = (prop || 0) + local;
	}
</script>

<p>{out}</p>
