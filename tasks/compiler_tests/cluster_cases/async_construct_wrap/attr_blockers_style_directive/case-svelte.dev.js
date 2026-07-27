import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <div></div>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var color, width;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => {
		color = "red";
		width = "1px";
	}]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	let styles;
	var div_1 = $.sibling(div, 2);
	let styles_1;
	$.template_effect(() => {
		styles = $.set_style(div, "", styles, { color });
		styles_1 = $.set_style(div_1, "", styles_1, { width });
	}, void 0, void 0, [$$promises[1], $$promises[1]]);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
