App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <br/>`, 1), App[$.FILENAME], [[5, 21]]);
var root_1 = $.add_locations($.from_html(`<!> <button>add</button>`, 1), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let array = $.tag_proxy($.proxy(["A"]), "array");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => array, $.index, ($$anchor, a) => {
		$.next();
		var fragment_1 = root();
		var text = $.first_child(fragment_1, true);
		$.next();
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return array.push("B");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
