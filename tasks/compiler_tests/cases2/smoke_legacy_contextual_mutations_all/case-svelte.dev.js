import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(` <button>snippet</button>`, 1), App[$.FILENAME], [[252, 1]]);
var root_1 = $.add_locations($.from_html(` <button>each-row</button>`, 1), App[$.FILENAME], [[224, 1]]);
var root_2 = $.add_locations($.from_html(` <!> <!> <!> <button>run</button>`, 1), App[$.FILENAME], [[300, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	const $store = () => ($.validate_store(store, "store"), $.store_get(store, "$store", $$stores));
	const $objStore = () => ($.validate_store(objStore, "objStore"), $.store_get(objStore, "$objStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const card = $.wrap_snippet(App, function($$anchor, param = $.noop) {
		$.validate_snippet_args(...arguments);
		$.next();
		var fragment = root();
		var text = $.first_child(fragment);
		var button = $.sibling(text);
		$.template_effect(() => $.set_text(text, `${param() ?? ""}
	${(param(), $.untrack(() => param().x)) ?? ""}
	${(param(), $.untrack(() => param().x = 1)) ?? ""}
	${(param(), $.untrack(() => param().x += 2)) ?? ""}
	${(param(), $.untrack(() => param().x++)) ?? ""}
	${(param(), $.untrack(() => ++param().x)) ?? ""}
	${(param(), $.untrack(() => param().x &&= 3)) ?? ""}
	${(param(), $.untrack(() => param().x ||= 4)) ?? ""}
	${(param(), $.untrack(() => param().x ??= 5)) ?? ""}
	${(param(), $.untrack(() => param()["x"] = 1)) ?? ""}
	${(param(), $.untrack(() => param()["x"]++)) ?? ""}
	${(param(), $.untrack(() => param()[key] = 1)) ?? ""}
	${(param(), $.untrack(() => param()[key]++)) ?? ""} `));
		$.delegated("click", button, function click() {
			param().x = 1;
			param().x += 2;
			param().x++;
			++param().x;
			param().x &&= 3;
			param().x ||= 4;
			param().x ??= 5;
			param()[key] = 1;
			param()[key]++;
		});
		$.append($$anchor, fragment);
	});
	let count = $.prop($$props, "count", 12, 0);
	let obj = $.prop($$props, "obj", 28, () => ({ x: 0 }));
	let local = $.tag($.mutable_source(0), "local");
	let localObj = $.tag($.mutable_source({ x: 0 }), "localObj");
	let key = "x";
	const store = writable(0);
	const objStore = writable({ x: 0 });
	let items = $.tag($.mutable_source([{
		id: 1,
		x: 0
	}, {
		id: 2,
		x: 0
	}]), "items");
	let promise = Promise.resolve({ x: 0 });
	function script_ops() {
		count(1);
		count(count() + 2);
		count(count() - 3);
		$.update_prop(count);
		$.update_prop(count, -1);
		$.update_pre_prop(count);
		$.update_pre_prop(count, -1);
		count(count() && 4);
		count(count() || 5);
		count(count() ?? 6);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x = 7, true), 30, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x += 8, true), 31, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x++, true), 32, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x--, true), 33, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(++obj().x, true), 34, 4);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(--obj().x, true), 35, 4);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x &&= 9, true), 36, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ||= 10, true), 37, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ??= 11, true), 38, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] = 7, true), 39, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] += 8, true), 40, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"]++, true), 41, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(++obj()["x"], true), 42, 4);
		$$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] = 7, true), 43, 2);
		$$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] += 8, true), 44, 2);
		$$ownership_validator.mutation(null, ["obj", key], obj(obj()[key]++, true), 45, 2);
		$$ownership_validator.mutation(null, ["obj", key], obj(++obj()[key], true), 46, 4);
		$.set(local, 12);
		$.set(local, $.get(local) + 13);
		$.set(local, $.get(local) - 14);
		$.update(local);
		$.update(local, -1);
		$.update_pre(local);
		$.update_pre(local, -1);
		$.set(local, $.get(local) && 15);
		$.set(local, $.get(local) || 16);
		$.set(local, $.get(local) ?? 17);
		$.mutate(localObj, $.get(localObj).x = 18);
		$.mutate(localObj, $.get(localObj).x += 19);
		$.mutate(localObj, $.get(localObj).x++);
		$.mutate(localObj, $.get(localObj).x--);
		$.mutate(localObj, ++$.get(localObj).x);
		$.mutate(localObj, --$.get(localObj).x);
		$.mutate(localObj, $.get(localObj).x &&= 20);
		$.mutate(localObj, $.get(localObj).x ||= 21);
		$.mutate(localObj, $.get(localObj).x ??= 22);
		$.mutate(localObj, $.get(localObj)["x"] = 18);
		$.mutate(localObj, $.get(localObj)["x"] += 19);
		$.mutate(localObj, $.get(localObj)["x"]++);
		$.mutate(localObj, ++$.get(localObj)["x"]);
		$.mutate(localObj, $.get(localObj)[key] = 18);
		$.mutate(localObj, $.get(localObj)[key] += 19);
		$.mutate(localObj, $.get(localObj)[key]++);
		$.mutate(localObj, ++$.get(localObj)[key]);
		$.store_set(store, 23);
		$.store_set(store, $store() + 24);
		$.store_set(store, $store() - 25);
		$.update_store(store, $store());
		$.update_store(store, $store(), -1);
		$.update_pre_store(store, $store());
		$.update_pre_store(store, $store(), -1);
		$.store_set(store, $store() && 26);
		$.store_set(store, $store() || 27);
		$.store_set(store, $store() ?? 28);
		$.store_mutate(objStore, $.untrack($objStore).x = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x &&= 31, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ||= 32, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ??= 33, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore));
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.next();
	var fragment_1 = root_2();
	var text_1 = $.first_child(fragment_1);
	var node = $.sibling(text_1);
	$.add_svelte_meta(() => $.each(node, 3, () => $.get(items), (item) => item.id, ($$anchor, item, i) => {
		$.next();
		var fragment_2 = root_1();
		var text_2 = $.first_child(fragment_2);
		var button_1 = $.sibling(text_2);
		$.template_effect(() => $.set_text(text_2, `${$.get(item) ?? ""}
	${$.get(i) ?? ""}
	${($.get(item), $.untrack(() => $.get(item).x)) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x = 1, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x += 2, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x -= 3, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x++, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x--, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => (++$.get(item).x, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => (--$.get(item).x, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x &&= 4, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x ||= 5, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item).x ??= 6, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item)["x"] = 1, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item)["x"] += 2, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item)["x"]++, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => (++$.get(item)["x"], $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item)[key] = 1, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item)[key] += 2, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => ($.get(item)[key]++, $.invalidate_inner_signals(() => $.get(items))))) ?? ""}
	${($.get(item), $.untrack(() => (++$.get(item)[key], $.invalidate_inner_signals(() => $.get(items))))) ?? ""} `));
		$.delegated("click", button_1, function click_1() {
			$.get(item).x = 1, $.invalidate_inner_signals(() => $.get(items));
			$.get(item).x += 2, $.invalidate_inner_signals(() => $.get(items));
			$.get(item).x++, $.invalidate_inner_signals(() => $.get(items));
			++$.get(item).x, $.invalidate_inner_signals(() => $.get(items));
			$.get(item).x &&= 4, $.invalidate_inner_signals(() => $.get(items));
			$.get(item).x ||= 5, $.invalidate_inner_signals(() => $.get(items));
			$.get(item).x ??= 6, $.invalidate_inner_signals(() => $.get(items));
			$.get(item)["x"] = 1, $.invalidate_inner_signals(() => $.get(items));
			$.get(item)[key] = 1, $.invalidate_inner_signals(() => $.get(items));
			$.get(item)[key]++, $.invalidate_inner_signals(() => $.get(items));
		});
		$.append($$anchor, fragment_2);
	}), "each", App, 202, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.each(node_1, 1, () => $.get(items), (it) => it.id, ($$anchor, it) => {
		const ctx = $.tag($.derived_safe_equal(() => $.get(it)), "ctx");
		$.get(ctx);
		$.next();
		var text_3 = $.text();
		$.template_effect(() => $.set_text(text_3, `${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x = 1)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x += 2)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x++)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => ++$.get(ctx).x)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x &&= 3)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x ||= 4)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx).x ??= 5)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx)["x"] = 1)) ?? ""}
	${($.deep_read_state($.get(ctx)), $.untrack(() => $.get(ctx)[key] = 1)) ?? ""}`));
		$.append($$anchor, text_3);
	}), "each", App, 265, 0);
	var node_2 = $.sibling(node_1, 2);
	$.add_svelte_meta(() => $.await(node_2, () => promise, null, ($$anchor, v) => {
		var text_4 = $.text();
		$.template_effect(() => $.set_text(text_4, `${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x = 1)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x += 2)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x++)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => ++$.get(v).x)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x &&= 3)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x ||= 4)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v).x ??= 5)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v)["x"] = 1)) ?? ""}
	${($.deep_read_state($.get(v)), $.untrack(() => $.get(v)[key] = 1)) ?? ""}`));
		$.append($$anchor, text_4);
	}, ($$anchor, e) => {
		var text_5 = $.text();
		$.template_effect(() => $.set_text(text_5, `${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).message)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).x = 1)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).x += 2)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e).x++)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => ++$.get(e).x)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e)["x"] = 1)) ?? ""}
	${($.deep_read_state($.get(e)), $.untrack(() => $.get(e)[key] = 1)) ?? ""}`));
		$.append($$anchor, text_5);
	}), "await", App, 279, 0);
	var button_2 = $.sibling(node_2, 2);
	$.template_effect(() => $.set_text(text_1, `${count() ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj().x)) ?? ""}
${$.get(local) ?? ""}
${($.get(localObj), $.untrack(() => $.get(localObj).x)) ?? ""}
${$store() ?? ""}
${($objStore(), $.untrack(() => $objStore().x)) ?? ""}

${($.deep_read_state(count()), $.untrack(() => count(1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() + 2))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() - 3))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_prop(count))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_prop(count, -1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_pre_prop(count))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_pre_prop(count, -1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() && 4))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() || 5))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() ?? 6))) ?? ""}

${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x = 7, true), 126, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x += 8, true), 127, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x++, true), 128, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x--, true), 129, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(++obj().x, true), 130, 3))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(--obj().x, true), 131, 3))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x &&= 9, true), 132, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ||= 10, true), 133, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ??= 11, true), 134, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] = 7, true), 135, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] += 8, true), 136, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"]++, true), 137, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(++obj()["x"], true), 138, 3))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] = 7, true), 139, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] += 8, true), 140, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(obj()[key]++, true), 141, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(++obj()[key], true), 142, 3))) ?? ""}

${($.get(local), $.untrack(() => $.set(local, 12))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) + 13))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) - 14))) ?? ""}
${($.get(local), $.untrack(() => $.update(local))) ?? ""}
${($.get(local), $.untrack(() => $.update(local, -1))) ?? ""}
${($.get(local), $.untrack(() => $.update_pre(local))) ?? ""}
${($.get(local), $.untrack(() => $.update_pre(local, -1))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) && 15))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) || 16))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) ?? 17))) ?? ""}

${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x--))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj).x))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, --$.get(localObj).x))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x &&= 20))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x ||= 21))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x ??= 22))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"] = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"] += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"]++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj)["x"]))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key] = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key] += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key]++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj)[key]))) ?? ""}

${($store(), $.untrack(() => $.store_set(store, 23))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() + 24))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() - 25))) ?? ""}
${($store(), $.untrack(() => $.update_store(store, $store()))) ?? ""}
${($store(), $.untrack(() => $.update_store(store, $store(), -1))) ?? ""}
${($store(), $.untrack(() => $.update_pre_store(store, $store()))) ?? ""}
${($store(), $.untrack(() => $.update_pre_store(store, $store(), -1))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() && 26))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() || 27))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() ?? 28))) ?? ""}

${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x &&= 31, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x ||= 32, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x ??= 33, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"] = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"] += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key] = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key] += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore)))) ?? ""} `));
	$.delegated("click", button_2, script_ops);
	$.append($$anchor, fragment_1);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
