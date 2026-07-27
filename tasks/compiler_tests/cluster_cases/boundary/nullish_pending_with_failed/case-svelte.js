import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let pending = null;
	let failed = () => {};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {
		pending,
		failed
	}, ($$anchor) => {
		$.next();
		var text = $.text("hi");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
