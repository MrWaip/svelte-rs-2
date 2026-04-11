<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		href?: string;
		target?: string;
		rel?: string;
		underline?: boolean;
		children?: Snippet;
	};

	let { href = '#', target = '_self', rel = '', underline = true, children }: Props = $props();
</script>

<a {href} {target} {rel} class="ui-link" style:text-decoration={underline ? 'underline' : 'none'}>
	{@render children?.()}
</a>

<style>
	.ui-link {
		color: #0070f3;
		cursor: pointer;
		transition: color 0.2s;
	}
	.ui-link:hover {
		color: #005bb5;
	}
</style>
