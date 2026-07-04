App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { token } from "./stores";
var root = $.add_locations($.from_html(`<p>cycle</p>`), App[$.FILENAME], [[5, 13]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $token = () => ($.validate_store(token, "token"), $.store_get(token, "$token", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.key(node, $token, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	}), "key", App, 5, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
