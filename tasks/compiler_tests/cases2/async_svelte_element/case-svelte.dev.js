import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>content</p>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function getTag() {
		return "div";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [async () => (await $.track_reactivity_loss(getTag()))()], (node, $$tag) => {
		$.validate_dynamic_element_tag(() => $.get($$tag));
		$.validate_void_dynamic_element(() => $.get($$tag));
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			var p = root();
			$.append($$anchor, p);
		}, void 0, [7, 0]);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
