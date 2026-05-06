import * as $ from "svelte/internal/client";
import { token } from "./stores";
var root_1 = $.from_html(`<p>cycle</p>`);
export default function App($$anchor) {
	const $token = () => $.store_get(token, "$token", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.key(node, $token, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
