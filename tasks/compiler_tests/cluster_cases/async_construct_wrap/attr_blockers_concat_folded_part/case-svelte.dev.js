import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a, b, c;
	var $$promises = $.run([
		async () => void (await $.track_reactivity_loss(Promise.resolve()))(),
		() => a = "a",
		async () => void (await $.track_reactivity_loss(Promise.resolve()))(),
		() => b = "b",
		async () => void (await $.track_reactivity_loss(Promise.resolve()))(),
		() => c = "c"
	]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => {
		styles = $.set_style(div, "w: a", styles, { color: c });
		$.set_attribute(div, "title", b);
	}, void 0, void 0, [
		$$promises[1],
		$$promises[5],
		$$promises[3]
	]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
