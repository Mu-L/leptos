# Mutating Data with Actions

We’ve talked about how to load `async` data with resources. Resources immediately load data and work closely with `<Suspense/>` and `<Transition/>` components to show whether data is loading in your app. But what if you just want to call some arbitrary `async` function and keep track of what it’s doing?

Well, you could always use [`spawn_local`](https://docs.rs/leptos/latest/leptos/fn.spawn_local.html). This allows you to just spawn an `async` task in a synchronous environment by handing the `Future` off to the browser (or, on the server, Tokio or whatever other runtime you’re using). But how do you know if it’s still pending? Well, you could just set a signal to show whether it’s loading, and another one to show the result...

All of this is true. Or you could use the final `async` primitive: [`create_action`](https://docs.rs/leptos/latest/leptos/fn.create_action.html).

Actions and resources seem similar, but they represent fundamentally different things. If you’re trying to load data by running an `async` function, either once or when some other value changes, you probably want to use `create_resource`. If you’re trying to occasionally run an `async` function in response to something like a user clicking a button, you probably want to use `create_action`.

Say we have some `async` function we want to run.

```rust
async fn add_todo(new_title: &str) -> Uuid {
    /* do some stuff on the server to add a new todo */
}
```

`create_action` takes a reactive `Scope` and an `async` function that takes a reference to a single argument, which you could think of as its “input type.”

> The input is always a single type. If you want to pass in multiple arguments, you can do it with a struct or tuple.
>
> ```rust
> // if there's a single argument, just use that
> let action1 = create_action(cx, |input: &String| {
>    let input = input.clone();
>    async move { todo!() }
> });
>
> // if there are no arguments, use the unit type `()`
> let action2 = create_action(cx, |input: &()| async { todo!() });
>
> // if there are multiple arguments, use a tuple
> let action3 = create_action(cx,
>   |input: &(usize, String)| async { todo!() }
> );
> ```
>
> Because the action function takes a reference but the `Future` needs to have a `'static` lifetime, you’ll usually need to clone the value to pass it into the `Future`. This is admittedly awkward but it unlocks some powerful features like optimistic UI. We’ll see a little more about that in future chapters.

So in this case, all we need to do to create an action is

```rust
let add_todo = create_action(cx, |input: &String| {
	let input = input.to_owned();
	async move { add_todo(&input).await }
});
```

Rather than calling `add_todo` directly, we’ll call it with `.dispatch()`, as in

```rust
add_todo.dispatch("Some value".to_string());
```

You can do this from an event listener, a timeout, or anywhere; because `.dispatch()` isn’t an `async` function, it can be called from a synchronous context.

Actions provide access to a few signals that synchronize between the asynchronous action you’re calling and the synchronous reactive system:

```rust
let submitted = add_todo.input(); // RwSignal<Option<String>>
let pending = add_todo.pending(); // ReadSignal<bool>
let todo_id = add_todo.value(); // RwSignal<Option<Uuid>>
```

This makes it easy to track the current state of your request, show a loading indicator, or do “optimistic UI” based on the assumption that the submission will succeed.

```rust
let input_ref = create_node_ref::<Input>(cx);

view! { cx,
	<form
		on:submit=move |ev| {
			ev.prevent_default(); // don't reload the page...
			let input = input_ref.get().expect("input to exist");
			add_todo.dispatch(input.value());
		}
	>
		<label>
			"What do you need to do?"
			<input type="text"
				node_ref=input_ref
			/>
		</label>
		<button type="submit">"Add Todo"</button>
	</form>
	// use our loading state
	<p>{move || pending().then("Loading...")}</p>
}
```

Now, there’s a chance this all seems a little over-complicated, or maybe too restricted. I wanted to include actions here, alongside resources, as the missing piece of the puzzle. In a real Leptos app, you’ll actually most often use actions alongside server functions, [`create_server_action`](https://docs.rs/leptos/latest/leptos/fn.create_server_action.html), and the [`<ActionForm/>`](https://docs.rs/leptos_router/latest/leptos_router/fn.ActionForm.html) component to create really powerful progressively-enhanced forms. So if this primitive seems useless to you... Don’t worry! Maybe it will make sense later. (Or check out our [`todo_app_sqlite`](https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite/src/todo.rs) example now.)

<iframe src="https://codesandbox.io/p/sandbox/10-async-resources-forked-hgpfp0?selection=%5B%7B%22endColumn%22%3A1%2C%22endLineNumber%22%3A4%2C%22startColumn%22%3A1%2C%22startLineNumber%22%3A4%7D%5D&file=%2Fsrc%2Fmain.rs" width="100%" height="1000px"></iframe>
