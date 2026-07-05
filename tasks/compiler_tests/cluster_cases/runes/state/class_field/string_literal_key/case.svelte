<script>
	class Toggle {
		"aria-pressed" = $state(false);

		toggle() {
			this["aria-pressed"] = !this["aria-pressed"];
		}
	}
	const toggle = new Toggle();
</script>

<button onclick={() => toggle.toggle()}>{toggle["aria-pressed"]}</button>
