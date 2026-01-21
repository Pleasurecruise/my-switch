import { createTRPCClient, httpBatchLink } from "@trpc/client";
import { createTRPCOptionsProxy } from "@trpc/tanstack-react-query";
import type { AppRouter } from "@my-monorepo/api/routers";
import superjson from "superjson";
import { createQueryClient } from "./query-client";

const queryClient = createQueryClient();

const trpcClient = createTRPCClient<AppRouter>({
	links: [
		httpBatchLink({
			// Use your local IP for mobile development
			// e.g., "http://192.168.1.100:3001/trpc"
			url: "http://localhost:3001/trpc",
			transformer: superjson,
		}),
	],
});

export const trpc = createTRPCOptionsProxy<AppRouter>({
	client: trpcClient,
	queryClient,
});

export { queryClient };
