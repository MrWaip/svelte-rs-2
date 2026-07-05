App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let p = $.tag_proxy($.proxy(Promise.resolve({
		"a-b": 1,
		"c d": 2
	})), "p");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { "a-b": ab, "c d": cd } = $.get($$source);
			return {
				ab,
				cd
			};
		});
		var ab = $.derived(() => $.get($$value).ab);
		var cd = $.derived(() => $.get($$value).cd);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
		$.append($$anchor, button);
	}), "await", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
