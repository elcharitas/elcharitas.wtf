"use client";
import { GraphQLClient, ClientContext } from "graphql-hooks";
import Blogs from "./blogs";

const client = new GraphQLClient({
  url: "https://api.hashnode.com",
  headers: {},
});

export default async function Page() {
  return (
    <ClientContext.Provider value={client}>
      <Blogs />
    </ClientContext.Provider>
  );
}
