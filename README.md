# resource-booking-zome

A bookings zome, and a resource mixin, to manage bookings to a community resource

## Backend

First version working, not yet polished.

### Building

This assumes you have the `holochain` and `dna-util` binaries installed as per [holochain-dna-build-tutorial](https://github.com/holochain/holochain-dna-build-tutorial).

```bash
cd dna
cargo build --release --target wasm32-unknown-unknown
dna-util -c bookings.dna.workdir/
```

## Testing

```bash
cd dna/tests
npm test
```

## Running

First, install [holochain-run-dna](https://www.npmjs.com/package/@holochain-open-dev/holochain-run-dna). Then run:

```bash
cd dna/
holochain-run-dna bookings.dna.gz
```

## UI

TODO
