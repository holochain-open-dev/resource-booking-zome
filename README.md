# Resource Bookings Module

> This repository is in its infancy and heavily construction, not yet ready for use.

Small zome to create and see calendar events, in holochain RSM.

This module is designed to be included in other DNAs, assuming as little as possible from those. It is packaged as a holochain zome, and an npm package that offers native Custom Elements that can be used across browsers and frameworks.

## Documentation

See our [`storybook`](https://holochain-open-dev.github.io/resource-bookings-module).

## Assumptions

These are the things you need to know to decide if you can use this module in your happ:

- Zome:
  - Optional dependency with the [`calendar_events` zome](https://github/holochain-open-dev/calendar-events-module).
- UI module:
  - `ApolloClient` as the state-management and data-fetching engine.
  - The resolvers are declared in the frontend using [`makeExecutableSchema`](https://www.npmjs.com/package/@graphql-tools/schema).
  - No framework or library assumed.

## Installation and usage

### Including the zome in your DNA

1. Create a new folder in the `zomes` of the consuming DNA, with the name you want to give to this zome in your DNA.
2. Add a new `Cargo.toml` in that folder. In its content, paste the `Cargo.toml` content from any zome.
3. Change the `name` properties of the `Cargo.toml` file to the name you want to give to this zome in your DNA.
4. Add this zome as a dependency in the `Cargo.toml` file:
```toml
[dependencies]
todo_rename_zome = {git = "https://github.com/holochain-open-dev/resource-bookings-module", package = "resource_bookings"}
```
5. Create a `src` folder besides the `Cargo.toml` with this content:
```rust
extern crate todo_rename_zome;
```
6. Add the zome into your `*.dna.workdir/dna.json` file.
7. Compile the DNA with the usual `CARGO_TARGET=target cargo build --release --target wasm32-unknown-unknown`.

### Using the UI module

1. Add it in your `package.json` with a reference to `holochain-open-dev/resource-bookings-module#ui-build`

2. Add the GraphQl schema and resolvers to your `ApolloClient` setup:

```js
import { AppWebsocket } from "@holochain/conductor-api";
import {
  resourceBookingsTypeDefs,
  resourceBookingsResolvers,
} from "@holochain-open-dev/resource-bookings";

export async function setupClient(url) {
  const appWebsocket = await AppWebsocket.connect(String(url));

  const appInfo = await appWebsocket.appInfo({ app_id: "test-app" });

  const cellId = appInfo.cell_data[0][0];

  const executableSchema = makeExecutableSchema({
    typeDefs: [rootTypeDef, resourceBookingsTypeDefs],
    resolvers: [resourceBookingsResolvers(appWebsocket, cellId)],
  });

  const schemaLink = new SchemaLink({ schema: executableSchema });

  return new ApolloClient({
    typeDefs: allTypeDefs,
    cache: new InMemoryCache(),
    link: schemaLink,
  });
}
```

3. In the root file of your application, install the module:

```js
import { ResourceBookingsModule } from "@holochain-open-dev/resource-bookings";
async function initApp() {
  const client = await setupClient(`ws://localhost:8888`);

  const calendarEventsModule = new CalendarEventsModule(client);

  await calendarEventsModule.install();
}
```

4. Once you have installed the module, all the elements you see in our storybook will become available for you to use in your HTML, like this:

```html
...
<body>
  <hod-cal-full-calendar></hod-cal-full-calendar>
</body>
```

Take into account that at this point the elements already expect a holochain conductor running at `ws://localhost:8888`.

## Developer setup

This respository is structured in the following way:

- `ui/`: UI library.
- `zome/`: example DNA with the `todo_rename_zome` code.
- Top level `Cargo.toml` is a virtual package necessary for other DNAs to include this zome by pointing to this git repository.

Read the [UI developer setup](/ui/README.md) and the [Zome developer setup](/zome/README.md).
