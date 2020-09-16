# resource-booking-zome
A bookings zome, and a resource mixin, to manage bookings to a community resource

## Backend

First version working, not yet polished.

### Building

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

## UI

TODO