import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>add</button>`, 1), App[$.FILENAME], [[11, 0], [12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let numbers = $.tag($.mutable_source([
		1,
		2,
		3
	]), "numbers");
	function add() {
		$.get(numbers).push($.get(numbers).length + 1);
		$.set(numbers, $.get(numbers));
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, ($.get(numbers), $.untrack(() => $.get(numbers).length))));
	$.delegated("click", button, add);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
