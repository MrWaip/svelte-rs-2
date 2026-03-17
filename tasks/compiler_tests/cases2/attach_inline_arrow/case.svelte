<script>
	let message = $state('hello');
</script>

<div {@attach (el) => {
	el.textContent = message;
}}>content</div>
