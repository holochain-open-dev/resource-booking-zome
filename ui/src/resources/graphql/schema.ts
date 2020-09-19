import { gql } from '@apollo/client/core';

export const resourcesTypeDefs = gql`
    interface Resource {
        id: ID!
        name: String!
        description: String!
    }
`;
