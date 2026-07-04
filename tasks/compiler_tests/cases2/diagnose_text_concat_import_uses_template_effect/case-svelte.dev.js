App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { NAME } from "./lib";
import Other from "./Other.svelte";
var root = $.add_locations($.from_html(`<div> <!></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div);
	var node = $.sibling(text);
	$.add_svelte_meta(() => Other(node, {}), "component", App, 6, 23, { componentTag: "Other" });
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Hello ${NAME ?? ""} world`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
