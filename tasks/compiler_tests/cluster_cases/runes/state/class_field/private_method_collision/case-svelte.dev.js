App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Repo {
		#_tree = $.tag($.state(), "Repo.tree");
		get tree() {
			return $.get(this.#_tree);
		}
		set tree(value) {
			$.set(this.#_tree, value, true);
		}
		async #tree() {}
	}
	const repo = new Repo();
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
