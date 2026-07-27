import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let pending = null;
	let failed = () => {};
	var $$exports = { ...$.legacy_api() };
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
	return $.pop($$exports);
}
