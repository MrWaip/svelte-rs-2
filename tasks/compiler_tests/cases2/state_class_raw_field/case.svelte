<script>
	class Store {
		data = $state.raw(null);
		update(val) {
			this.data = val;
		}
	}
	let s = new Store();
</script>

<p>{s.data}</p>
