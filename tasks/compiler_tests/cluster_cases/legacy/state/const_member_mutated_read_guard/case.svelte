<script>
	const store = { state: { show: true } };
	const close = () => {
		store.state.show = false;
	};
</script>

<button onclick={close}>{store.state.show}</button>
