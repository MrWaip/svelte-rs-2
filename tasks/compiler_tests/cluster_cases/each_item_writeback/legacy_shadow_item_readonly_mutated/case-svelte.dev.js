import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <button>go</button>`, 1), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(["Hello"]), "a");
	function go() {
		$.set(a, [...$.get(a), "x"]);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(a), $.index, ($$anchor, a, $$index, $$array) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, text);
	}), "each", App, 9, 0);
	var button = $.sibling(node, 2);
	$.event("click", button, go);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
