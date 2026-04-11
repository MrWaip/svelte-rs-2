<script lang="ts">
	import type { Component } from 'svelte';
	type Props = {
		value?: string;
		placeholder?: string;
		type?: 'text' | 'password' | 'email' | 'search' | 'number' | 'tel' | 'url';
		disabled?: boolean;
		icon?: Component;
	};

	let {
		value = $bindable(''),
		placeholder = '',
		type = 'text',
		disabled = false,
		icon: Icon
	}: Props = $props();
</script>

<div class="ui-input-wrapper">
	{#if Icon}
		<div class="ui-input-icon">
			<Icon />
		</div>
	{/if}
	<input {value} {placeholder} {type} {disabled} class="ui-input" />
</div>

<style>
	.ui-input-wrapper {
		display: flex;
		align-items: center;
		position: relative;
	}
	.ui-input-icon {
		margin-right: 0.7em;
		width: 1.3em;
		height: 1.3em;
		flex-shrink: 0;
		color: #0070f3;
		opacity: 0.85;
	}
	.ui-input {
		flex: 1 1 0;
		padding: 0.5em 1em;
		border: 1px solid #ccc;
		border-radius: 4px;
		font-size: 1em;
		outline: none;
		transition: border 0.2s;
		background: #fff;
		color: #222;
	}
	.ui-input:focus {
		border-color: #0070f3;
	}
	.ui-input[disabled] {
		background: #f3f3f3;
		color: #aaa;
		border-color: #e0e0e0;
		cursor: not-allowed;
		opacity: 0.7;
	}
</style>
