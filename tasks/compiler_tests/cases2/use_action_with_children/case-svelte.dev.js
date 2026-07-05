App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.add_locations($.from_html(`<label><input type="checkbox"/> <span> </span></label>`), App[$.FILENAME], [[
	6,
	0,
	[[7, 1], [8, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var label = root();
	var span = $.sibling($.child(label), 2);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(label);
	$.action(label, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => $$props.config);
	$.template_effect(() => $.set_text(text, $$props.value));
	$.append($$anchor, label);
	return $.pop($$exports);
}
