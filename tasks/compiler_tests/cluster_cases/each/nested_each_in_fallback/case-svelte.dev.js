import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[2, 14]]);
var root_1 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[2, 45]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag_proxy($.proxy([]), "a");
	let b = $.tag_proxy($.proxy([]), "b");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => a, $.index, ($$anchor, x) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(x)));
		$.append($$anchor, p);
	}, ($$anchor) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 17, () => b, $.index, ($$anchor, x, $$index, $$array) => {
			var span = root_1();
			var text_1 = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text_1, $.get(x)));
			$.append($$anchor, span);
		}), "each", App, 2, 31);
		$.append($$anchor, fragment_1);
	}), "each", App, 2, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
