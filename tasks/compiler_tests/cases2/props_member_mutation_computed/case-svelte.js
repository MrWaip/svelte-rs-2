import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let user = $.prop($$props, "user", 23, () => ({ profile: {} }));
	let key = "name";
	function rename() {
		user().profile[key] = "next";
	}
	$.pop();
}
