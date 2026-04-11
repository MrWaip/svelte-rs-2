<script lang="ts">
	type Props = {
		checked?: boolean;
		disabled?: boolean;
	};

	let { checked = $bindable(false), disabled = false }: Props = $props();
</script>

<input type="checkbox" bind:checked {disabled} />
