App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<li> </li>`), App[$.FILENAME], [[15, 8]]);
var root_1 = $.add_locations($.from_html(`<button> </button> <p></p> <ul></ul>`, 1), App[$.FILENAME], [
	[11, 0],
	[12, 0],
	[13, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	let label = "hello";
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	function increment() {
		$.set(count, $.get(count) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var p = $.sibling(button, 2);
	p.textContent = "hello";
	var ul = $.sibling(p, 2);
	$.add_svelte_meta(() => $.each(ul, 21, () => items, $.index, ($$anchor, item) => {
		var li = root();
		var text_1 = $.child(li, true);
		$.reset(li);
		$.template_effect(() => $.set_text(text_1, $.get(item)));
		$.append($$anchor, li);
	}), "each", App, 14, 4);
	$.reset(ul);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, increment);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
