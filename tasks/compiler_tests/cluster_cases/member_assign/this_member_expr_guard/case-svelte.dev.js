App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function action(node) {
		return { update(count) {
			console.log(...$.log_if_contains_state("log", "update", this.count, this.count = count));
		} };
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.action(button, ($$node) => action?.($$node));
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
