// src/routes/index.tsx
import * as fs from "node:fs";
import { createFileRoute, useRouter } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { useQuery } from "@tanstack/react-query";
import { Button } from "@my-monorepo/ui/components/button";
import { trpc } from "@/src/lib/trpc";

const filePath = "count.txt";

async function readCount() {
	return parseInt(
		await fs.promises.readFile(filePath, "utf-8").catch(() => "0"),
		10,
	);
}

const getCount = createServerFn({
	method: "GET",
}).handler(() => {
	return readCount();
});

const updateCount = createServerFn({ method: "POST" })
	.inputValidator((d: number) => d)
	.handler(async ({ data }) => {
		const count = await readCount();
		await fs.promises.writeFile(filePath, `${count + data}`);
	});

export const Route = createFileRoute("/")({
	component: Home,
	loader: async () => await getCount(),
});

function Home() {
	const router = useRouter();
	const state = Route.useLoaderData();

	const { data: helloData, isLoading } = useQuery(
		trpc.hello.greet.queryOptions(),
	);

	return (
		<div className="flex flex-col gap-4 p-4">
			<div className="p-4 border rounded">
				<h2 className="text-lg font-bold mb-2">tRPC Test</h2>
				{isLoading ? <p>Loading...</p> : <p>{helloData?.message}</p>}
			</div>

			<Button
				type="button"
				onClick={() => {
					updateCount({ data: 1 }).then(() => {
						router.invalidate();
					});
				}}
			>
				Add 1 to {state}?
			</Button>
		</div>
	);
}
