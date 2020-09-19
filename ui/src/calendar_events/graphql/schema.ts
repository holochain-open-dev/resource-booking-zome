import { gql } from '@apollo/client';

export const calendarEventsTypeDefs = gql`
    scalar Date

    extend type Query {
        allCalendarEvents: [CalendarEvent!]!
    }

    type CalendarEvent {
        id: ID!
        startTime: Date!
        endTime: Date!
        createdBy: ID!
        location: ID
        invitiees: [ID!]!
    }
`;
