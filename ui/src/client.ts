import { ApolloClient, gql, InMemoryCache } from '@apollo/client';
import { SchemaLink } from '@apollo/client/link/schema';
import { makeExecutableSchema } from '@graphql-tools/schema';

import { AppWebsocket, CellId } from '@holochain/conductor-api';

import { bookingsTypeDefs } from './bookings/graphql/schema';
import { calendarEventsResolvers } from './calendar_events/graphql/resolvers';
import { calendarEventsTypeDefs } from './calendar_events/graphql/schema';
import { meetingRoomsTypeDefs } from './meeting_rooms/graphql/schema';
import { resourcesTypeDefs } from './resources/graphql/schema';

const rootTypeDef = gql`
  type Query {
    _: Boolean
  }

  type Mutation {
    _: Boolean
  }
`;

const allTypeDefs = [
  rootTypeDef,
  calendarEventsTypeDefs,
  bookingsTypeDefs,
  meetingRoomsTypeDefs,
  resourcesTypeDefs,
];

async function setupClient(url: String, cellId: CellId) {
  const appWebsocket = await AppWebsocket.connect(String(url));

  const executableSchema = makeExecutableSchema({
    typeDefs: allTypeDefs,
    resolvers: [calendarEventsResolvers(appWebsocket, cellId)],
  });

  const schemaLink = new SchemaLink({ schema: executableSchema });

  return new ApolloClient({
    typeDefs: allTypeDefs,

    cache: new InMemoryCache(),
    link: schemaLink,
  });
}
