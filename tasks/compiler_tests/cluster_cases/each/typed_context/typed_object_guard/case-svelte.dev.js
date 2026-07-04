App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<button>add</button> <!>`, 1), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{ a: 1 }]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let a = () => $.get($$item).a;
		a();
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, a()));
		$.append($$anchor, span);
	}), "each", App, 5, 0);
	$.delegated("click", button, function click() {
		return items.push({ a: items.length });
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
