import { Resolvers } from '@apollo/client/core';
import { AppWebsocket, CellId } from '@holochain/conductor-api';
import { Buffer } from 'buffer';

export const calendarEventsResolvers = (
  appWebsocket: AppWebsocket,
  cellId: CellId
): Resolvers => ({
  Query: {
    async allCalendarEvents() {
      return appWebsocket.callZome({
        cap: Buffer.from(Array(64).fill('aa') as any, 'hex'),
        cell_id: cellId,
        zome_name: 'calendar_events',
        fn_name: 'get_all_calendar_events',
        payload: null,
        provenance: cellId[1],
      });
    },
  },
});
