import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let isActive = false;
	const bold = true;
	$$renderer.push(`<div${$.attr_class($.clsx({
		active: isActive,
		bold
	}))}>content</div>`);
}
