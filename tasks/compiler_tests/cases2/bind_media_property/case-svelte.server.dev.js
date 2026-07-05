App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let duration = 0;
		let videoWidth = 0;
		let videoHeight = 0;
		$$renderer.push(`<audio>`);
		$.push_element($$renderer, "audio", 7, 0);
		$$renderer.push(`</audio>`);
		$.pop_element();
		$$renderer.push(` <video>`);
		$.push_element($$renderer, "video", 9, 0);
		$$renderer.push(`</video>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
