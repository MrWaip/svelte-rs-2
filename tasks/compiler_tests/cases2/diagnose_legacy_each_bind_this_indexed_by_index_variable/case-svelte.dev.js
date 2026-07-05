import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	let refs = $.tag($.mutable_source([]), "refs");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item, idx) => {
		var input = root();
		$.bind_this(input, ($$value, idx) => $.mutate(refs, $.get(refs)[idx] = $$value), (idx) => $.get(refs)?.[idx], () => [idx]);
		$.append($$anchor, input);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
