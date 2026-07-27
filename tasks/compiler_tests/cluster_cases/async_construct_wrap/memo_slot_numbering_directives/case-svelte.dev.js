import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>x</div>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn() {
		return "red";
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	let styles;
	$.template_effect(($0, $1) => {
		classes = $.set_class(div, 1, "", null, classes, $1);
		styles = $.set_style(div, "", styles, $0);
	}, [() => ({ color: fn() })], [async () => ({ a: (await $.track_reactivity_loss(true))() })]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
