import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(null), "a");
	let b = null;
	let c = null;
	function load(source) {
		(($$value) => {
			var $$array = $.to_array($$value, 3);
			$.set(a, $$array[0]);
			b = $$array[1];
			c = $$array[2];
		})(source());
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.delegated("click", button, function click() {
		return load(() => [
			1,
			2,
			3
		]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
