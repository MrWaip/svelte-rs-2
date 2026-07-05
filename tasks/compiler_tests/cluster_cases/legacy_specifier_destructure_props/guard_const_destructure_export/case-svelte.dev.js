import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const { i, j } = {
		i: 1,
		j: 2
	};
	var $$exports = {
		...$.legacy_api(),
		get i() {
			return i;
		}
	};
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${i ?? ""}${j ?? ""}`));
	$.append($$anchor, p);
	$.bind_prop($$props, "i", i);
	return $.pop($$exports);
}
