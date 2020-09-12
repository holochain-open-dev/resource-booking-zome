```js script
import { html } from '@open-wc/demoing-storybook';
import '../dist/hod-resource-bookings.js';

export default {
  title: 'HodResourceBookings',
  component: 'hod-resource-bookings',
  options: { selectedPanel: "storybookjs/knobs/panel" },
};
```

# HodResourceBookings

A component for...

## Features:

- a
- b
- ...

## How to use

### Installation

```bash
yarn add hod-resource-bookings
```

```js
import 'hod-resource-bookings/hod-resource-bookings.js';
```

```js preview-story
export const Simple = () => html`
  <hod-resource-bookings></hod-resource-bookings>
`;
```

## Variations

###### Custom Title

```js preview-story
export const CustomTitle = () => html`
  <hod-resource-bookings title="Hello World"></hod-resource-bookings>
`;
```
