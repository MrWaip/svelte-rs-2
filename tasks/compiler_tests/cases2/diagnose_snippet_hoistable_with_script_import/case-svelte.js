import * as $ from "svelte/internal/client";
import { noop } from "./helpers.js";
const socket = ($$anchor) => {
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, noop));
	$.append($$anchor, div);
};
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	socket($$anchor);
}
