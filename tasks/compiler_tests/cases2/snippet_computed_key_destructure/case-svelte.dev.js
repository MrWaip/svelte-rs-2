App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[13, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const view = $.wrap_snippet(App, function($$anchor, $$arg0) {
		$.validate_snippet_args(...arguments);
		let value = () => ($$arg0?.())[key()];
		value();
		let rest = () => $.exclude_from_object($$arg0?.(), [String(key())]);
		rest();
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${value() ?? ""} ${rest().extra ?? ""}`));
		$.append($$anchor, p);
	});
	let data = $.tag_proxy($.proxy({
		label: "world",
		extra: "ok"
	}), "data");
	function key() {
		return "label";
	}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => view($$anchor, () => data), "render", App, 16, 0);
	return $.pop($$exports);
}
