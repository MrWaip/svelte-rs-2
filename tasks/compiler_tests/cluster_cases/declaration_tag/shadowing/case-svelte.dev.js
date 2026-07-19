App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <span> </span>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => [1, 2], $.index, ($$anchor, n, $$index, $$array) => {
		const value = n * 10;
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, value));
		$.append($$anchor, p);
	}), "each", App, 4, 0);
	var span = $.sibling(node, 2);
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, $$props.value));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
