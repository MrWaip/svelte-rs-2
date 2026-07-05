App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let p = $.tag_proxy($.proxy(Promise.resolve({ outer: [{ inner: 1 }] })), "p");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { outer: [{ inner }] } = $.get($$source);
			return { inner };
		});
		var inner = $.derived(() => $.get($$value).inner);
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(inner)));
		$.append($$anchor, button);
	}), "await", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
