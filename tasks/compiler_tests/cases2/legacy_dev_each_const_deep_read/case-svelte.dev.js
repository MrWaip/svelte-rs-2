import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[13, 4]]);
var root_1 = $.add_locations($.from_html(`<button>add</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.tag($.mutable_source([{ name: "a" }, { name: "b" }]), "rows");
	function add() {
		$.set(rows, [...$.get(rows), { name: "c" }]);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(rows), $.index, ($$anchor, row, idx) => {
		const label = $.tag($.derived_safe_equal(() => ($.get(row), idx, $.untrack(() => $.get(row).name + idx))), "label");
		$.get(label);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(label)));
		$.append($$anchor, p);
	}), "each", App, 11, 0);
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
