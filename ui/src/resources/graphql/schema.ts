import { gql } from '@apollo/client';

export const resourcesTypeDefs = gql`
    interface Resource {
        id: ID!
        name: String!
        description: String!
    }
`;
