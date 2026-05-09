import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.from_html(`<label><input type="checkbox"/> <span> </span></label>`);
export default function App($$anchor, $$props) {
	var label = root();
	var span = $.sibling($.child(label), 2);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(label);
	$.action(label, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => $$props.config);
	$.template_effect(() => $.set_text(text, $$props.value));
	$.append($$anchor, label);
}
