import { gql } from '@apollo/client/core';

export const bookingsTypeDefs = gql`

    extend type Query {
        bookingRequestForResource(resourceId: ID!): [BookingRequest!]!
        bookingsForResource(resourceId: ID!): [BookingRequest!]!
    }

    type BookingRequest {
        id: ID!
        resource: Resource!
        requestor: ID!
        startTime: Date!
        endTime: Date!
    }

    type Booking {
        id: ID!
        request: BookingRequest!
        createdAt: Date!
    }
`;
