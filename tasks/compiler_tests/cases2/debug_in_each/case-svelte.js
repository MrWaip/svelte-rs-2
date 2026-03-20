App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = [
		1,
		2,
		3
	];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, item) => {
		$.template_effect(() => {
			console.log({ item: $.snapshot($.get(item)) });
			debugger;
		});
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
