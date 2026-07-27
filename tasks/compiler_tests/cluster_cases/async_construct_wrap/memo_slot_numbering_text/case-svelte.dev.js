import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div><div>y</div>`, 1), App[$.FILENAME], [[2, 0], [2, 22]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div, true);
	$.reset(div);
	var div_1 = $.sibling(div);
	$.template_effect(($0, $1) => {
		$.set_text(text, $1);
		$.set_attribute(div_1, "id", $0);
	}, [() => fn()], [async () => (await $.track_reactivity_loss("x"))()]);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
