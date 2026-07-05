import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let className = $.prop($$props, "class", 8, "btn");
	function getClass() {
		return className();
	}
	var $$exports = {
		...$.legacy_api(),
		get getClass() {
			return getClass;
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, className()));
	$.append($$anchor, p);
	$.bind_prop($$props, "getClass", getClass);
	return $.pop($$exports);
}
