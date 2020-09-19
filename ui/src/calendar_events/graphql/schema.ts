import { gql } from '@apollo/client';

export const calendarEventsTypeDefs = gql`
    scalar Date

    extend type Query {
        allCalendarEvents: [CalendarEvent!]!
    }

    type CalendarEvent {
        id: ID!
        title: String!
        startTime: Date!
        endTime: Date!
        createdBy: ID!
        location: ID
        invitiees: [ID!]!
    }
`;
