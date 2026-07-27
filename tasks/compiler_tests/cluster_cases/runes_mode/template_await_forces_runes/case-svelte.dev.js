import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <button>inc</button>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss(compute(count)))()]);
	$.delegated("click", button, function click() {
		return count++;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
