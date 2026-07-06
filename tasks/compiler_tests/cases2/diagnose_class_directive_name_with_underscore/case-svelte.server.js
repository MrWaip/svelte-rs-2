import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { isExpanded } = $$props;
	$$renderer.push(`<div${$.attr_class("", void 0, { "is_expanded": isExpanded })}>hi</div>`);
}
