import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-1mj6a7z">inside</div> <div data-state="closed">outside</div>`, 1);
export default function App($$anchor) {
	let open = true;
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "data-state", open ? "open" : "closed");
	$.next(2);
	$.append($$anchor, fragment);
}
