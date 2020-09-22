import { gql } from '@apollo/client/core';
import { setupClient } from '../dist';

setupClient('ws://localhost:8888').then(async client => {
  const result = await client.query({
    query: gql`
      {
        allCalendarEvents {
          id
          title
        }
      }
    `,
  });

  console.log(result);
});
