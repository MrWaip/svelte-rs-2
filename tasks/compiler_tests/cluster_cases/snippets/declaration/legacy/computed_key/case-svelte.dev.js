import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const s = $.wrap_snippet(App, function($$anchor, $$arg0) {
		$.validate_snippet_args(...arguments);
		let v = () => ($$arg0?.())[k];
		v();
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, v()));
		$.append($$anchor, button);
	});
	const k = "z";
	let v = { z: 1 };
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => s($$anchor, () => v), "render", App, 9, 0);
	return $.pop($$exports);
}
