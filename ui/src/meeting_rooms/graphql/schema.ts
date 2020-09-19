import { gql } from '@apollo/client/core';

export const meetingRoomsTypeDefs = gql`
    extend type Query {
        allMeetingRooms: [MeetingRoom!]!
    }

    type MeetingRoom implements Resource {
        id: ID!
        name: String!
        description: String!
        owner: ID!
    }
`;
