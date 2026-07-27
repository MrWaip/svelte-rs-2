import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, c } = $$props;
		$$renderer.child(async ($$renderer) => {
			const [$$0, $$1] = (await $.save(Promise.all([(async () => (await $.save(a))())(), (async () => (await $.save(c))())()])))();
			$$renderer.option({ class: $$1 }, $$0);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
