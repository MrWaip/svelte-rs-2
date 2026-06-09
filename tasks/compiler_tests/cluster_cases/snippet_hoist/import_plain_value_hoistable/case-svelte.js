import * as $ from "svelte/internal/client";
import { noop } from "./helpers.js";
const foo = ($$anchor) => {
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, noop));
	$.append($$anchor, text);
};
export default function App($$anchor) {
	foo($$anchor);
}
