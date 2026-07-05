import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function greet() {
		return "hi";
	}
	var $$exports = {
		...$.legacy_api(),
		get greet() {
			return greet;
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.untrack(greet)]);
	$.append($$anchor, p);
	$.bind_prop($$props, "greet", greet);
	return $.pop($$exports);
}
