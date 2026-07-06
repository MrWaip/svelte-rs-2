import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let isButton = $$props["isButton"];
		let onAction = $$props["onAction"];
		let item = $$props["item"];
		Child($$renderer, { onclick: isButton ? () => onAction(item) : undefined });
		$.bind_props($$props, {
			isButton,
			onAction,
			item
		});
	});
}
