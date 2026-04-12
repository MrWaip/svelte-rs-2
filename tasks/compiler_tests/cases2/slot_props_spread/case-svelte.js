import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let props = { foo: "bar" };
	let item = "hello";
	let extra = "world";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "footer", $.spread_props({
		item,
		extra
	}, () => props), null);
	$.append($$anchor, fragment);
}
