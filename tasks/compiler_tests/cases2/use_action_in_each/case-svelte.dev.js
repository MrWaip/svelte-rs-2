App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.add_locations($.from_html(`<div>text</div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		"a",
		"b",
		"c"
	]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, item) => {
		var div = root();
		$.action(div, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => $.get(item));
		$.append($$anchor, div);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
