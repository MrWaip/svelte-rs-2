import * as $ from "svelte/internal/client";
import { noop } from "./helpers.js";
const a = ($$anchor) => {
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, noop));
	$.append($$anchor, text);
};
const b = ($$anchor) => {
	a($$anchor);
};
export default function App($$anchor) {
	b($$anchor);
}
