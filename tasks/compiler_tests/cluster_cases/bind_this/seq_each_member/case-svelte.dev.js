App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<b> </b>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let arr = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "arr");
	let elements = $.tag_proxy($.proxy([]), "elements");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 18, () => arr, (item) => item, ($$anchor, item, i) => {
		var b = root();
		var text = $.child(b, true);
		$.reset(b);
		$.bind_this(b, (v, i) => elements[i] = v, (i) => elements[i], () => [$.get(i)]);
		$.template_effect(() => $.set_text(text, item));
		$.append($$anchor, b);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
