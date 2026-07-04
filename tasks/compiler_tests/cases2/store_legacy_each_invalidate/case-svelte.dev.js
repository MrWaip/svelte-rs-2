import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { items } from "./stores";
var root = $.add_locations($.from_html(`<p> </p> <button>edit</button>`, 1), App[$.FILENAME], [[6, 1], [7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $items = () => ($.validate_store(items, "items"), $.store_get(items, "$items", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, $items, $.index, ($$anchor, item, $$index) => {
		var fragment_1 = root();
		var p = $.first_child(fragment_1);
		var text = $.child(p, true);
		$.reset(p);
		var button = $.sibling(p, 2);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).name))));
		$.delegated("click", button, function click() {
			return $.get(item).name = "x", $.invalidate_inner_signals(() => $items()), $.invalidate_store($$stores, "$items");
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
