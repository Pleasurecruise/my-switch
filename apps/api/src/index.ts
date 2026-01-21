import { Hono } from "hono";
import { cors } from "hono/cors";
import { trpcServer } from "@hono/trpc-server";
import { appRouter } from "./trpc/routers";
import { createTRPCContext } from "./trpc/init";

const app = new Hono();

app.use(
	"*",
	cors({
		origin: [
			"http://localhost:3000", // web
			"http://localhost:1420", // tauri
		],
		allowMethods: ["GET", "POST", "OPTIONS"],
		allowHeaders: ["Content-Type"],
	}),
);

app.use(
	"/trpc/*",
	trpcServer({
		router: appRouter,
		createContext: createTRPCContext,
	}),
);

app.get("/", (c) => {
	return c.json({ message: "tRPC API Server" });
});

const port = Number(process.env.PORT) || 3001;

console.log(`Server running on http://localhost:${port}`);

export default {
	port,
	fetch: app.fetch,
};
